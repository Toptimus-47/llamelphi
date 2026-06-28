# MAGI 2026: FULL AUTOMATION SUITE
# This script orchestrates the Sidecar, Core Backend, and Integration Tests.

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "   MAGI 2026: UNIFIED AUTOMATION SYSTEM   " -ForegroundColor White -BackgroundColor Blue
Write-Host "==========================================================" -ForegroundColor Cyan

# 0. Environment Setup Check
Write-Host "[1/5] Checking Dependencies..." -ForegroundColor Yellow
if (-not (Get-Command "python" -ErrorAction SilentlyContinue)) { Write-Error "Python not found."; exit 1 }
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) { Write-Error "Rust/Cargo not found."; exit 1 }

# 1. Start Web Search Sidecar (Playwright)
Write-Host "[2/5] Starting Web Search Sidecar (Port 8001)..." -ForegroundColor Yellow
$sidecarProcess = Start-Process -FilePath "python" -ArgumentList "web_search_sidecar.py" -NoNewWindow -PassThru

# 2. Start MAGI Core (Rust)
Write-Host "[3/5] Starting MAGI Core Backend (Port 8000)..." -ForegroundColor Yellow
Push-Location "magi_core"
# Use 'cargo run --bin magi_server' as requested for automation
$coreProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--bin", "magi_server" -NoNewWindow -PassThru
Pop-Location

# 3. Wait for Services to be Ready
Write-Host "[4/5] Waiting for services to initialize..." -ForegroundColor Yellow
$retries = 30
$ready = $false
while ($retries -gt 0 -and -not $ready) {
    try {
        $sidecarCheck = Invoke-WebRequest -Uri "http://127.0.0.1:8001/docs" -Method Get -TimeoutSec 1 -ErrorAction SilentlyContinue
        $coreCheck = Invoke-WebRequest -Uri "http://127.0.0.1:8000/" -Method Get -TimeoutSec 1 -ErrorAction SilentlyContinue
        
        if ($sidecarCheck.StatusCode -eq 200 -and $coreCheck.StatusCode -eq 200) {
            $ready = $true
            Write-Host "[+] All systems online." -ForegroundColor Green
        }
    } catch {
        # Not ready yet
    }
    $retries--
    Start-Sleep -Seconds 2
    Write-Host "." -NoNewline -ForegroundColor Gray
}

if (-not $ready) {
    Write-Host "`n[X] ERROR: Services failed to start in time." -ForegroundColor Red
    # Cleanup before exit
    Stop-Process -Id $sidecarProcess.Id -Force -ErrorAction SilentlyContinue
    Stop-Process -Id $coreProcess.Id -Force -ErrorAction SilentlyContinue
    exit 1
}

# 4. Run Integration Test Suite
Write-Host "`n[5/5] Executing Integration Test Suite..." -ForegroundColor Yellow
Write-Host "----------------------------------------------------------"
python integration_test_suite.py
$testExitCode = $LASTEXITCODE

# 5. Cleanup or Keep Alive
Write-Host "----------------------------------------------------------"
Write-Host "Testing finished with Exit Code: $testExitCode" -ForegroundColor Cyan

if ($testExitCode -eq 0) {
    Write-Host "   [ ALL TESTS PASSED ]   " -ForegroundColor Black -BackgroundColor Green
} else {
    Write-Host "   [ TESTS FAILED ]   " -ForegroundColor White -BackgroundColor Red
}

Write-Host ""
Write-Host "MAGI System is still running in the background." -ForegroundColor Gray
Write-Host "Press any key to shutdown ALL MAGI services and exit..." -ForegroundColor Yellow
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

Write-Host "[*] Shutting down Sidecar (PID: $($sidecarProcess.Id))..." -ForegroundColor Gray
Stop-Process -Id $sidecarProcess.Id -Force -ErrorAction SilentlyContinue

Write-Host "[*] Shutting down MAGI Core (PID: $($coreProcess.Id))..." -ForegroundColor Gray
# Stopping 'cargo run' might leave the compiled binary running, so we kill by port or name as well
Stop-Process -Id $coreProcess.Id -Force -ErrorAction SilentlyContinue
Get-Process "magi_server" -ErrorAction SilentlyContinue | Stop-Process -Force

Write-Host "[+] All services terminated. Goodbye." -ForegroundColor Cyan
