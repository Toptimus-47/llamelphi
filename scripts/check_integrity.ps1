# ELITE Integrity Check Script
# Purpose: Prevent sensitive data exposure in Git

$sensitivePatterns = @("knowledge_*.md", "vector_db/*.json", "sessions/*.log", "prompts/sensitive_*.txt")
$violationFound = $false

Write-Host "[*] Starting ELITE Integrity Check..." -ForegroundColor Cyan

# 1. Check for staged sensitive files
$stagedFiles = git diff --cached --name-only
foreach ($file in $stagedFiles) {
    foreach ($pattern in $sensitivePatterns) {
        if ($file -like $pattern) {
            Write-Host "[!] CRITICAL: Sensitive file '$file' is staged for commit!" -ForegroundColor Red
            $violationFound = $true
        }
    }
}

# 2. Check .gitignore coverage
$gitignoreContent = Get-Content .gitignore
foreach ($pattern in $sensitivePatterns) {
    $found = $false
    foreach ($line in $gitignoreContent) {
        if ($line -eq $pattern -or $line -eq $pattern.Replace("*", "")) {
            $found = $true
            break
        }
    }
    if (-not $found) {
        Write-Host "[!] WARNING: Pattern '$pattern' might not be fully covered in .gitignore" -ForegroundColor Yellow
    }
}

if ($violationFound) {
    Write-Host "[X] Integrity check FAILED. Please unstage sensitive files immediately." -ForegroundColor Red
    exit 1
} else {
    Write-Host "[V] Integrity check PASSED. No sensitive data exposure detected in staged changes." -ForegroundColor Green
    exit 0
}
