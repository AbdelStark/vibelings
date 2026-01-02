# vibelings install script for Windows
# Usage: irm https://raw.githubusercontent.com/AbdelStark/vibelings/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "AbdelStark/vibelings"
$BinaryName = "vibelings"
$InstallDir = if ($env:VIBELINGS_INSTALL_DIR) { $env:VIBELINGS_INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }

function Write-Info {
    param([string]$Message)
    Write-Host "info: " -ForegroundColor Blue -NoNewline
    Write-Host $Message
}

function Write-Success {
    param([string]$Message)
    Write-Host "success: " -ForegroundColor Green -NoNewline
    Write-Host $Message
}

function Write-Warn {
    param([string]$Message)
    Write-Host "warning: " -ForegroundColor Yellow -NoNewline
    Write-Host $Message
}

function Write-Error {
    param([string]$Message)
    Write-Host "error: " -ForegroundColor Red -NoNewline
    Write-Host $Message
    exit 1
}

function Get-Platform {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture

    switch ($arch) {
        "X64" { return "$BinaryName-windows-x86_64" }
        default { Write-Error "Unsupported architecture: $arch" }
    }
}

function Get-LatestVersion {
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        return $response.tag_name
    }
    catch {
        return $null
    }
}

function Install-FromBinary {
    param(
        [string]$Platform,
        [string]$Version
    )

    $url = "https://github.com/$Repo/releases/download/$Version/$Platform.zip"
    $tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    $zipPath = Join-Path $tempDir "vibelings.zip"

    try {
        Write-Info "Downloading vibelings $Version for $Platform..."
        Invoke-WebRequest -Uri $url -OutFile $zipPath

        Write-Info "Extracting..."
        Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

        # Create install directory
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }

        # Copy binary
        $binaryPath = Join-Path $tempDir "$Platform.exe"
        if (-not (Test-Path $binaryPath)) {
            $binaryPath = Join-Path $tempDir "$BinaryName.exe"
        }

        if (Test-Path $binaryPath) {
            Copy-Item $binaryPath -Destination (Join-Path $InstallDir "$BinaryName.exe") -Force
            return $true
        }
        return $false
    }
    catch {
        return $false
    }
    finally {
        if (Test-Path $tempDir) {
            Remove-Item -Recurse -Force $tempDir
        }
    }
}

function Install-FromCargo {
    Write-Info "Installing from source using cargo..."

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if (-not $cargo) {
        Write-Error "cargo not found. Please install Rust: https://rustup.rs"
    }

    cargo install --git "https://github.com/$Repo"
}

function Add-ToPath {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        Write-Warn "$InstallDir is not in your PATH"
        Write-Host ""
        Write-Host "To add it permanently, run:"
        Write-Host ""
        Write-Host "    `$env:Path += `";$InstallDir`""
        Write-Host "    [Environment]::SetEnvironmentVariable(`"Path`", `$env:Path + `";$InstallDir`", `"User`")"
        Write-Host ""
    }
}

function Main {
    Write-Host ""
    Write-Host "  vibelings installer"
    Write-Host "  Rustlings for agentic programming"
    Write-Host ""

    $platform = Get-Platform
    Write-Info "Detected platform: $platform"

    $version = Get-LatestVersion
    if ($version) {
        Write-Info "Latest version: $version"

        if (Install-FromBinary -Platform $platform -Version $version) {
            Write-Success "vibelings installed to $InstallDir\$BinaryName.exe"
            Add-ToPath

            Write-Host ""
            Write-Success "Installation complete!"
            Write-Host ""
            Write-Host "Get started:"
            Write-Host "    vibelings init"
            Write-Host "    vibelings list"
            Write-Host "    vibelings"
            Write-Host ""
            exit 0
        }
        else {
            Write-Warn "Could not download pre-built binary, falling back to cargo install"
        }
    }
    else {
        Write-Warn "Could not fetch latest release, falling back to cargo install"
    }

    Install-FromCargo

    Write-Success "vibelings installed via cargo"
    Write-Host ""
    Write-Host "Get started:"
    Write-Host "    vibelings init"
    Write-Host "    vibelings list"
    Write-Host "    vibelings"
    Write-Host ""
}

Main
