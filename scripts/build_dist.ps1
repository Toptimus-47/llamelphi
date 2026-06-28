# MAGI 2026: Standalone Build & Local CI Script
# Goal: Build Rust backend, Flutter frontend, and bundle them for local use.

$ErrorActionPreference = "Stop"

Write-Host ">>> [CI/CD] Starting Local Build Pipeline..." -ForegroundColor Cyan

# MAGI 2026: Standalone Build & Local CI Script (Integrated Edition)
$ErrorActionPreference = "Stop"

Write-Host ">>> [CI/CD] Starting Integrated Build Pipeline..." -ForegroundColor Cyan

# Define Local Tools
$localFlutter = Join-Path (Get-Location) "flutter_sdk\bin\flutter.bat"
if (-not (Test-Path $localFlutter)) {
    Write-Host "[!] Local Flutter SDK not found at $localFlutter. Falling back to system 'flutter'." -ForegroundColor Gray
    $localFlutter = "flutter"
}

# 1. Backend Build (Rust - DLL for FFI)
Write-Host ">>> [1/3] Building Rust Dynamic Library (magi_core)..." -ForegroundColor Yellow
Set-Location "magi_core"
cargo build --release
if ($LASTEXITCODE -ne 0) { Write-Host "Backend Build Failed!" -ForegroundColor Red; exit 1 }
Set-Location ".."

# 2. Frontend Build (Flutter)
Write-Host ">>> [2/3] Building Flutter Frontend (magi_gui)..." -ForegroundColor Yellow
Set-Location "magi_gui"
& $localFlutter build windows --release
if ($LASTEXITCODE -ne 0) { Write-Host "Frontend Build Failed!" -ForegroundColor Red; exit 1 }
Set-Location ".."

# 3. Bundling (Integrated Binary Distribution)
Write-Host ">>> [3/3] Creating Integrated Bundle..." -ForegroundColor Yellow
$distDir = "dist"
if (Test-Path $distDir) { Remove-Item -Recurse $distDir }
New-Item -ItemType Directory -Path $distDir

# Copy Flutter Build Output (The actual GUI)
$flutterBuildPath = "magi_gui\build\windows\x64\runner\Release"
if (Test-Path $flutterBuildPath) {
    Copy-Item -Path "$flutterBuildPath\*" -Destination $distDir -Recurse
    Write-Host "[+] GUI artifacts copied to $distDir" -ForegroundColor Green
} else {
    Write-Host "[X] Error: Flutter build output not found at $flutterBuildPath" -ForegroundColor Red
    exit 1
}

# Copy Backend DLL (Used by FFI)
# Note: FFI bridge in Dart looks for it in root or relative path. 
# We copy it to the root of dist so it's next to the .exe
Copy-Item "magi_core/target/release/magi_core.dll" -Destination "$distDir/magi_core.dll"

# Copy Models & Prompts
New-Item -ItemType Directory -Path "$distDir/models"
if (Test-Path "models/*.json") { Copy-Item "models/*.json" -Destination "$distDir/models/" }
Copy-Item -Recurse "prompts" -Destination "$distDir/prompts"
if (Test-Path "magi_config.ini") { Copy-Item "magi_config.ini" -Destination "$distDir/magi_config.ini" }

# Create Launch Script
$launchScript = @"
@echo off
setlocal
cd /d %~dp0
echo ==========================================
echo    MAGI Research Terminal - Integrated
echo ==========================================
echo.
echo Launching GUI (Backend initialized via FFI)...
start magi_gui.exe
echo.
echo [DONE] System is running in background.
"@
$launchScript | Out-File -FilePath "$distDir/launch_magi.bat" -Encoding ascii

Write-Host ">>> [SUCCESS] Integrated Bundle ready in /dist" -ForegroundColor Green
Write-Host "Run ./dist/launch_magi.bat to start the system." -ForegroundColor Cyan

Write-Host ">>> [SUCCESS] Local CI Pipeline Completed. Bundle ready in /dist" -ForegroundColor Green
