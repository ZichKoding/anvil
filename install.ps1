#Requires -Version 5.1
$ErrorActionPreference = 'Stop'

$Repo = "ZichKoding/anvil"
$Binary = "anvil.exe"

# Detect architecture
$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
switch ($arch) {
    'X64'   { $target = "x86_64-pc-windows-msvc" }
    default { Write-Error "Unsupported architecture: $arch"; exit 1 }
}

Write-Host "Detected platform: $target"

# Fetch latest release tag
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
$tag = $release.tag_name

if (-not $tag) {
    Write-Error "Could not determine latest release"
    exit 1
}

Write-Host "Latest release: $tag"

$archive = "anvil-$tag-$target.zip"
$url = "https://github.com/$Repo/releases/download/$tag/$archive"

# Download to temp
$tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ("anvil-install-" + [System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null

try {
    $zipPath = Join-Path $tmpDir $archive

    Write-Host "Downloading $url..."
    Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing

    Write-Host "Extracting..."
    Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

    # Install binary
    $installDir = Join-Path $env:USERPROFILE ".local\bin"
    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    }

    Copy-Item (Join-Path $tmpDir $Binary) (Join-Path $installDir $Binary) -Force
    Write-Host "Installed $Binary to $installDir\$Binary"

    # Add to user PATH if not already present
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
        $env:Path = "$env:Path;$installDir"
        Write-Host "Added $installDir to user PATH (restart your terminal for it to take effect)"
    }

    # Verify
    $installed = Join-Path $installDir $Binary
    if (Test-Path $installed) {
        Write-Host "Success! Run 'anvil' to get started."
    }
}
finally {
    Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
}
