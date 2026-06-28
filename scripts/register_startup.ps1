# MAGI Startup Registration Script
# Run this once to create a proper Windows Startup shortcut.

$targetScript = "E:\llmassist\upload\launch_gemini_startup.ps1"
$shortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\LaunchMAGI.lnk"

try {
    $WshShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut($shortcutPath)
    $Shortcut.TargetPath = "powershell.exe"
    # Use backticks to escape quotes for the file path in arguments
    $Shortcut.Arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$targetScript`""
    $Shortcut.WorkingDirectory = "E:\llmassist\upload"
    $Shortcut.Description = "Launch Gemini CLI for MAGI Project"
    $Shortcut.Save()

    Write-Host "==========================================" -ForegroundColor Cyan
    Write-Host "[+] SUCCESS: Gemini CLI registered to Startup." -ForegroundColor Green
    Write-Host "[+] Shortcut created: $shortcutPath" -ForegroundColor White
    Write-Host "==========================================" -ForegroundColor Cyan
} catch {
    Write-Host "[X] ERROR: Failed to create startup shortcut." -ForegroundColor Red
    Write-Host $_.Exception.Message
}
