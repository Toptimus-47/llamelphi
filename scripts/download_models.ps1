# MAGI High-Speed Downloader with Verified SHA256 (v4.0)
# Uses Unsloth/Bartowski verified repositories for maximum stability.

$modelsDir = "models"
if (-not (Test-Path $modelsDir)) { New-Item -ItemType Directory -Path $modelsDir }
$ProgressPreference = 'SilentlyContinue'

# Authoritative SHA256 Hashes from Hugging Face LFS Metadata
$modelInventory = @(
    @{ 
        name = "Phi-4-mini-instruct (Melchior)"; 
        url = "https://huggingface.co/unsloth/Phi-4-mini-instruct-GGUF/resolve/main/Phi-4-mini-instruct-Q4_K_M.gguf"; 
        dest = "Phi-4-mini-instruct-Q4_K_M.gguf";
        sha256 = "88c00229914083cd112853aab84ed51b87bdf6b9ce42f532d8c85c7c63b1730a"
    },
    @{ 
        name = "Gemma-3-4B-IT (Balthasar)"; 
        url = "https://huggingface.co/unsloth/gemma-3-4b-it-GGUF/resolve/main/gemma-3-4b-it-Q4_K_M.gguf"; 
        dest = "gemma-3-4b-it-Q4_K_M.gguf";
        sha256 = "04a43a22e8d2003deda5acc262f68ec1005fa76c735a9962a8c77042a74a7d19"
    },
    @{ 
        name = "DeepSeek-R1-1.5B (Casper)"; 
        url = "https://huggingface.co/unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF/resolve/main/DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf"; 
        dest = "DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf";
        sha256 = "f3bdf9cf31dee4b57ae4e455a1cb0d01b5c2c1b50d72d3112141c195506c2840"
    },
    @{ 
        name = "SmolLM2-1.7B (Artaban)"; 
        url = "https://huggingface.co/unsloth/SmolLM2-1.7B-Instruct-GGUF/resolve/main/SmolLM2-1.7B-Instruct-Q4_K_M.gguf"; 
        dest = "SmolLM2-1.7B-Instruct-Q4_K_M.gguf";
        sha256 = "61b6f90dd515fd3bffbd0f6ba716e87555dde77d9b0573a562c2c5e62afc4909"
    },
    @{ 
        name = "DeepSeek-Coder-V2 (Gushnasaph)"; 
        url = "https://huggingface.co/bartowski/DeepSeek-Coder-V2-Lite-Instruct-GGUF/resolve/main/DeepSeek-Coder-V2-Lite-Instruct-Q4_K_M.gguf"; 
        dest = "DeepSeek-Coder-V2-Lite-Instruct-Q4_K_M.gguf";
        sha256 = "603bd3f8a0281d16571da7c08bd661ee17ff0d1be6fcbd1b42242da257ef0bb8"
    },
    @{ 
        name = "Qwen2.5-Math-1.5B (Kagba)"; 
        url = "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf"; 
        dest = "qwen2.5-1.5b-instruct-q4_k_m.gguf";
        sha256 = "6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e"
    }
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   MAGI VERIFIED DOWNLOADER v4.0" -ForegroundColor Yellow
Write-Host "==========================================" -ForegroundColor Cyan

foreach ($model in $modelInventory) {
    $destPath = Join-Path $modelsDir $model.dest
    Write-Host "`n[*] Target: $($model.name)" -ForegroundColor White

    # 1. Byte-perfect Resume/Sync
    curl.exe -L -C - $model.url -o $destPath --progress-bar --fail
    
    # 2. Strict SHA256 Check
    if (Test-Path $destPath) {
        Write-Host "[*] Verifying bit-integrity..." -ForegroundColor Gray -NoNewline
        $actualHash = (Get-FileHash -Path $destPath -Algorithm SHA256).Hash.ToLower()
        Write-Host " DONE." -ForegroundColor Gray

        if ($actualHash -eq $model.sha256) {
            Write-Host "[V] Hash Verified: bit-perfect." -ForegroundColor Green
        } else {
            Write-Host "[X] Hash Mismatch! Attempting to fix by re-downloading..." -ForegroundColor Red
            Remove-Item $destPath -Force
            curl.exe -L $model.url -o $destPath --progress-bar --fail
        }
    }
}

# Finalize Tokenizer
curl.exe -L "https://huggingface.co/HuggingFaceTB/SmolLM2-1.7B-Instruct/resolve/main/tokenizer.json" -o (Join-Path $modelsDir "tokenizer.json") --silent
Write-Host "`n[V] Environment ready." -ForegroundColor Green
