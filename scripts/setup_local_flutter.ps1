# MAGI Local Flutter Setup Script (v1.0)
# Purpose: Detects a Flutter SDK zip in the root, extracts it, and links it to the system.

$ErrorActionPreference = "Stop"
$destFolder = "flutter_sdk"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   MAGI: LOCAL SDK AUTO-CONFIGURATION" -ForegroundColor Yellow
Write-Host "==========================================" -ForegroundColor Cyan

# 1. Detect Zip File
$zipFile = Get-ChildItem -Path . -Filter "flutter_windows_*.zip" | Select-Object -First 1

if (-not $zipFile) {
    Write-Host "[X] Error: No Flutter SDK zip file found in the root directory." -ForegroundColor Red
    Write-Host "Please ensure your zip file (e.g., flutter_windows_3.22.2-stable.zip) is in this folder." -ForegroundColor Gray
    exit
}

Write-Host "[*] Detected SDK Zip: $($zipFile.Name)" -ForegroundColor Green

# 2. Extract SDK
if (Test-Path $destFolder) {
    Write-Host "[!] Target folder '$destFolder' already exists. Re-extracting..." -ForegroundColor Yellow
    Remove-Item $destFolder -Recurse -Force
}

Write-Host "[*] Extracting Flutter SDK... This may take 2-3 minutes." -ForegroundColor Magenta
Expand-Archive -Path $zipFile.FullName -DestinationPath "temp_extract"

# Flutter zip usually contains a top-level 'flutter' folder
if (Test-Path "temp_extract/flutter") {
    Move-Item "temp_extract/flutter" $destFolder
    Remove-Item "temp_extract" -Recurse -Force
} else {
    Move-Item "temp_extract" $destFolder
}

Write-Host "[+] SDK successfully extracted to '$destFolder'." -ForegroundColor Green

# 3. Environment Check
$flutterBat = Join-Path (Get-Location) "$destFolder\bin\flutter.bat"
if (Test-Path $flutterBat) {
    Write-Host "[*] Verifying SDK path: $flutterBat" -ForegroundColor Gray
    # Try running --version to initialize
    & $flutterBat --version
    Write-Host "[V] Local Flutter SDK is ready for use." -ForegroundColor Green
} else {
    Write-Host "[X] Error: flutter.bat not found in the expected location." -ForegroundColor Red
    exit
}

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   SYSTEM READY: Run ./start_magi.ps1 now" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan
