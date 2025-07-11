function rustpoint {
    $exePath = Join-Path $PSScriptRoot "target\debug\rustpoint.exe"
    $path = & $exePath --mode use
    if ($LASTEXITCODE -eq 0 -and $path) {
        Set-Location $path
    }
    elseif ($LASTEXITCODE -ne 0) {
        Write-Host "Rustpoint failed to select a path. Make sure you have added paths."
    }
    else {
        Write-Host "No path was selected."
    }
}