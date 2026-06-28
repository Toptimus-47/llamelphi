# llamelphi: MAGI Smart Launcher (v1.3)
# Intelligent fallback: Uses binary if exists, otherwise uses flutter run.

$ErrorActionPreference = "Continue" # Don't stop if one check fails
$CORE_BIN = "magi_core\target\release\magi_server.exe"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   MAGI SYSTEM: SMART LAUNCHER" -ForegroundColor Yellow
Write-Host "==========================================" -ForegroundColor Cyan

# 1. Start MAGI_Core (Rust)
if (-not (Test-Path $CORE_BIN)) {
    Write-Host "[*] Building MAGI_Core (Release)..." -ForegroundColor Magenta
    Push-Location "magi_core"
    cargo build --release
    Pop-Location
}

Write-Host "[+] Launching MAGI_Core Server..." -ForegroundColor Green
$backendProcess = Start-Process -FilePath $CORE_BIN -NoNewWindow -PassThru -ErrorAction SilentlyContinue

if (-not $backendProcess) {
    Write-Host "[!] Failed to launch magi_server.exe. Is it already running?" -ForegroundColor Yellow
}

# 2. Start MAGI_Terminal (Flutter)
# Look for pre-built binary
$guiBin = Get-ChildItem -Path "magi_gui\build\windows\x64\runner\Release\*.exe" -ErrorAction SilentlyContinue | Select-Object -First 1

if ($guiBin) {
    Write-Host "[+] Found built binary: $($guiBin.Name). Launching..." -ForegroundColor Green
    Start-Process -FilePath $guiBin.FullName
} else {
    Write-Host "[*] Release binary not found. Launching via 'flutter run' (Development Mode)..." -ForegroundColor Magenta
    Write-Host "[!] This may take a few seconds to initialize." -ForegroundColor Gray
    Push-Location "magi_gui"
    Start-Process -FilePath "cmd.exe" -ArgumentList "/c flutter run -d windows"
    Pop-Location
}

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   SYSTEM ACTIVE" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Press any key to shutdown ALL MAGI services..." -ForegroundColor Yellow

# Wait for exit signal
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# 3. Cleanup
Write-Host "[*] Initiating Shutdown Protocol..." -ForegroundColor Red
if ($backendProcess) {
    Write-Host "[*] Terminating MAGI_Core (PID: $($backendProcess.Id))..." -ForegroundColor Gray
    Stop-Process -Id $backendProcess.Id -Force -ErrorAction SilentlyContinue
}

# Also try to kill any stray flutter processes if they were started in this session
Get-Process "magi_gui" -ErrorAction SilentlyContinue | Stop-Process -Force

Write-Host "[+] MAGI System safely offline." -ForegroundColor Cyan
