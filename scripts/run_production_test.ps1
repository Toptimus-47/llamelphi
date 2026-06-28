# MAGI High-Performance Integration Test Runner
# This script optimizes the environment for multi-agent load testing.

Write-Host "--- [MAGI] Initializing Production-Ready Test Environment ---" -ForegroundColor Cyan

# 1. Configuration Optimization: Set Max Active Models to 6
$mainFs = "magi_core/src/main.rs"
$content = Get-Content $mainFs
$newContent = $content -replace 'ResourceManager::new\(3\)', 'ResourceManager::new(6)'
$newContent | Set-Content $mainFs
Write-Host "[+] ResourceManager optimized: Max active models set to 6 (No Swapping)." -ForegroundColor Green

# 2. Build and Start Server in Release Mode
Write-Host "[*] Building MAGI Server in Release mode (This may take a few minutes)..." -ForegroundColor Yellow
pushd magi_core
cargo build --release --bin magi_server
if ($LASTEXITCODE -ne 0) {
    Write-Host "[!] Build failed. Aborting." -ForegroundColor Red
    popd
    exit
}

Write-Host "[+] Build successful. Starting MAGI Server..." -ForegroundColor Green
$serverProc = Start-Process -FilePath "target/release/magi_server.exe" -NoNewWindow -PassThru
popd

# 3. Wait for Initialization
Write-Host "[*] Waiting 15 seconds for models and embedder to initialize..." -ForegroundColor Yellow
Start-Sleep -Seconds 15

# 4. Execute Security Audit Scenario
Write-Host "--- [MAGI] Executing Project Aegis: Security Audit Scenario ---" -ForegroundColor Cyan
python scenario_runner.py

# 5. Cleanup
Write-Host "--- [MAGI] Cleaning up ---" -ForegroundColor Cyan
Stop-Process -Id $serverProc.Id -Force
$newContent = $content -replace 'ResourceManager::new\(6\)', 'ResourceManager::new(3)'
$newContent | Set-Content $mainFs
Write-Host "[+] Environment restored to default." -ForegroundColor Green
Write-Host "[!] Test Session Concluded." -ForegroundColor Cyan
