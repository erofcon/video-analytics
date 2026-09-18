$ErrorActionPreference = "Stop"

$vsDevShell = "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1"

if (-not (Test-Path $vsDevShell)) {
    throw "Visual Studio Developer PowerShell not found: $vsDevShell"
}

& $vsDevShell -Arch amd64 -HostArch amd64 -SkipAutomaticLocation

$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

Write-Host ""
Write-Host "Development environment initialized." -ForegroundColor Green