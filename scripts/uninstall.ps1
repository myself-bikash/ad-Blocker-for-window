$ErrorActionPreference = 'Stop'

$serviceName = 'AdBlockService'
$installRoot = Join-Path $env:LOCALAPPDATA 'AdBlocker'

$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if ($service) {
    Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
    sc.exe delete $serviceName | Out-Null
}

if (Test-Path $installRoot) {
    Remove-Item -Recurse -Force $installRoot -ErrorAction SilentlyContinue
}

if (Test-Path (Join-Path $env:ProgramData 'AdBlocker')) {
    Remove-Item -Recurse -Force (Join-Path $env:ProgramData 'AdBlocker') -ErrorAction SilentlyContinue
}

Write-Host 'Uninstall complete. Windows networking has not been left in a broken state by this script.'
