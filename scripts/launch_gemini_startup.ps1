# MAGI Environment: Auto-Launch Script
# This script is intended to be linked to Windows Startup.

$projectRoot = "E:\llmassist\upload"
$geminiCmd = "C:\Users\r1256\AppData\Roaming\npm\gemini.cmd"

if (Test-Path $projectRoot) {
    Set-Location $projectRoot
    Write-Host ">>> Launching Gemini CLI in MAGI Project Root..." -ForegroundColor Cyan
    & $geminiCmd
} else {
    Write-Host "[X] Error: Project root not found at $projectRoot" -ForegroundColor Red
    Pause
}
