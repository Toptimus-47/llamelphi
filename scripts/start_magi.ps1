# MAGI SYSTEM: ROBUST STANDALONE LAUNCHER (v1.7)
# Purpose: Enforces fresh build, provides diagnostic guidance for Flutter.

$ErrorActionPreference = "Continue"
$CORE_DIR = "magi_core"
$GUI_DIR = "magi_gui"
$BIN_PATH = "$CORE_DIR\target\release\magi_server.exe"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   MAGI_SYSTEM: ROBUST STARTUP ENGINE" -ForegroundColor Yellow
Write-Host "==========================================" -ForegroundColor Cyan

# 1. Backend Build
Write-Host "[*] Phase 1: Validating Backend Binary..." -ForegroundColor Green
if (Test-Path $BIN_PATH) { Remove-Item $BIN_PATH -Force }

Write-Host "[*] Building MAGI_Core (Release)..." -ForegroundColor Magenta
Push-Location $CORE_DIR
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "[X] Backend build failed." -ForegroundColor Red
    exit
}
Pop-Location

# 2. Logger-First Launch
Write-Host "[*] Phase 2: Initializing Real-time Logger..." -ForegroundColor Green
$today = Get-Date -Format "yyyy-MM-dd"
$logFile = "logs/magi_system.log.$today"

# Ensure log directory and file exist for tailing
if (-not (Test-Path "logs")) { New-Item -ItemType Directory -Path "logs" | Out-Null }
if (-not (Test-Path $logFile)) { New-Item -ItemType File -Path $logFile | Out-Null }

# Launch dedicated log monitor window
Start-Process powershell -ArgumentList "-NoProfile -Command Write-Host '>>> MAGI REAL-TIME SYSTEM LOG MONITOR <<<' -ForegroundColor Yellow; Get-Content -Path '$logFile' -Wait -Tail 20"

Write-Host "[+] Launching MAGI_Core Server in background..." -ForegroundColor Green
$backendProcess = Start-Process -FilePath $BIN_PATH -NoNewWindow -PassThru -ErrorAction SilentlyContinue

# 3. Frontend Check & Launch
Write-Host "[*] Phase 3: Launching MAGI_Terminal..." -ForegroundColor Green

# Priority: 1. Local flutter_sdk, 2. System PATH, 3. Manual Input
$localFlutter = Join-Path (Get-Location) "flutter_sdk\bin\flutter.bat"
$flutterCmd = Get-Command flutter -ErrorAction SilentlyContinue

if (Test-Path $localFlutter) {
    $flutterExe = $localFlutter
    Write-Host "[+] Using local Flutter SDK found in flutter_sdk folder." -ForegroundColor Green
} elseif ($flutterCmd) {
    $flutterExe = "flutter"
    Write-Host "[+] Using system-wide Flutter command." -ForegroundColor Green
} else {
    Write-Host "[!] WARNING: 'flutter' command not found." -ForegroundColor Yellow
    Write-Host "Please provide the FULL PATH to your flutter.bat (e.g., E:\llmassist\upload\flutter_sdk\bin\flutter.bat):" -ForegroundColor White
    $userPath = Read-Host "Path"
    if (Test-Path $userPath) {
        $flutterExe = $userPath
    } else {
        Write-Host "[X] Invalid path. Cannot launch GUI automatically." -ForegroundColor Red
        return # Use return instead of goto for cleaner exit
    }
}

Write-Host "[+] Initializing Flutter GUI..." -ForegroundColor Magenta
Push-Location $GUI_DIR
Start-Process -FilePath "cmd.exe" -ArgumentList "/k $flutterExe run -d windows"
Pop-Location

# System online indicator
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   SYSTEM ACTIVE | BACKEND: ONLINE" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ">>> Press any key to safely SHUTDOWN all MAGI services <<<" -ForegroundColor Yellow

$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# 4. Cleanup
Write-Host "`n[*] Initiating Shutdown Protocol..." -ForegroundColor Red
if ($backendProcess) { Stop-Process -Id $backendProcess.Id -Force -ErrorAction SilentlyContinue }
Get-Process "magi_server" -ErrorAction SilentlyContinue | Stop-Process -Force
Get-Process "magi_gui" -ErrorAction SilentlyContinue | Stop-Process -Force
Write-Host "[+] MAGI System safely offline." -ForegroundColor Cyan
