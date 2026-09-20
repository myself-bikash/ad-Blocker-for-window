$serviceName = 'AdBlockService'
$service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
if (-not $service) {
    throw 'AdBlockService is not installed.'
}

Start-Service -Name $serviceName -ErrorAction Stop
Write-Host 'AdBlockService enabled.'
