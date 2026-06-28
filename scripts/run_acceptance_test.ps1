# MAGI 2026: Acceptance Test Suite
# This script verifies the Transparent Intelligence UX and Adversarial Peer-Review logic.

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "   MAGI 2026 SYSTEM ACCEPTANCE TEST   " -ForegroundColor White -BackgroundColor Blue
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host ""

$projectRoot = Get-Location
$corePath = Join-Path $projectRoot "magi_core"

# 1. Compilation Check
Write-Host "[1/3] Verifying System Integrity (Compiling)..." -ForegroundColor Yellow
Set-Location $corePath
cargo build --quiet
if ($LASTEXITCODE -ne 0) {
    Write-Host "[X] ERROR: Compilation failed." -ForegroundColor Red
    exit 1
}
Write-Host "[+] Integrity Verified." -ForegroundColor Green
Write-Host ""

# 2. Execute Adversarial Consensus Test (Clean Architecture Case)
Write-Host "[2/3] Executing Adversarial Peer-Review Logic Test..." -ForegroundColor Yellow
Write-Host "Scenario: 'Explain the significance of Clean Architecture'" -ForegroundColor Gray
Write-Host "----------------------------------------------------------"

# Run the test and capture output
$testOutput = cargo test --lib application::consensus::engine::tests::test_clean_architecture_explanation_refinement -- --nocapture 2>&1

# 3. Parse and Display Procedure Highlights
Write-Host "[3/3] Procedure Transparency Report:" -ForegroundColor Yellow
Write-Host ""

$hasProcedure = $false
$hasTelemetry = $false
$hasReasoning = $false
$hasCritique = $false

foreach ($lineObj in $testOutput) {
    $line = $lineObj.ToString()

    # Highlight Search Strategy
    if ($line -like "*Search keywords candidates identified*") {
        Write-Host ">>> [PROCEDURE] Search Strategy Formulated:" -ForegroundColor Cyan
        Write-Host "    $line" -ForegroundColor White
        $hasProcedure = $true
    }
    
    # Highlight Telemetry
    if ($line -like "*telemetry*" -and $line -like "*metrics*") {
        if (-not $hasTelemetry) {
            Write-Host ">>> [TELEMETRY] Real-time Data Gathering Started..." -ForegroundColor Green
            $hasTelemetry = $true
        }
        Write-Host "    $line" -ForegroundColor Gray
    }

    # Highlight Reasoning
    if ($line -like "*REASONING*" -or $line -like "*thought process*") {
        if (-not $hasReasoning) {
            Write-Host ">>> [REASONING] Adversarial Auditor Engaging..." -ForegroundColor Magenta
            $hasReasoning = $true
        }
        Write-Host "    $line" -ForegroundColor Gray
    }

    # Highlight Critiques
    if ($line -like "*(Critic)*") {
        Write-Host ">>> [ADVERSARIAL] Critique Received: $line" -ForegroundColor Red
        $hasCritique = $true
    }

    # Show Progress Messages
    if ($line.StartsWith("[MAGI]")) {
        Write-Host "$line" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "   FINAL VERDICT   " -ForegroundColor White

if ($hasProcedure -and $hasReasoning -and $hasCritique) {
    Write-Host "   [ PASS ]   " -ForegroundColor Black -BackgroundColor Green
    Write-Host "All transparent intelligence procedures are operational." -ForegroundColor Green
} else {
    Write-Host "   [ FAIL ]   " -ForegroundColor White -BackgroundColor Red
    Write-Host "Some procedure elements were missing in the stream." -ForegroundColor Red
}
Write-Host "==========================================================" -ForegroundColor Cyan

Set-Location $projectRoot
