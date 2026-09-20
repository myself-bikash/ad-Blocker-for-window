$serviceName = 'AdBlockService'
$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if (-not $service) {
    throw 'AdBlockService is not installed.'
}

Stop-Service -Name $serviceName -Force -ErrorAction Stop
Write-Host 'AdBlockService disabled.'
