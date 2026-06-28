@echo off
SETLOCAL EnableDelayedExpansion

:: ==========================================
::   MAGI SYSTEM: TURN-KEY STARTER (v2.0)
:: ==========================================

echo [*] Checking for required components...

:: 1. Check for Models
IF NOT EXIST "models\Phi-4-mini-instruct-Q4_K_M.gguf" (
    echo [!] Missing models detected. Starting high-speed downloader...
    powershell -NoProfile -ExecutionPolicy Bypass -File "scripts\download_models.ps1"
    IF %ERRORLEVEL% NEQ 0 (
        echo [X] Model download failed. Please check your internet connection.
        pause
        exit /b %ERRORLEVEL%
    )
) ELSE (
    echo [V] Models found in models/ directory.
)

:: 2. Check for Tokenizer
IF NOT EXIST "models\tokenizer.json" (
    echo [!] Tokenizer missing. Downloading...
    powershell -NoProfile -Command "Invoke-WebRequest -Uri 'https://huggingface.co/HuggingFaceTB/SmolLM2-1.7B-Instruct/resolve/main/tokenizer.json' -OutFile 'models\tokenizer.json'"
)

:: 3. Launch Integrated Suite
echo [*] Launching MAGI Integrated System...
powershell -NoProfile -ExecutionPolicy Bypass -File "scripts\start_magi.ps1"

IF %ERRORLEVEL% NEQ 0 (
    echo [X] System failed to start. Check logs for details.
    pause
)

ENDLOCAL
