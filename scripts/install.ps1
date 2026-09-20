$ErrorActionPreference = 'Stop'

$serviceName = 'AdBlockService'
$programFiles = if ($env:ProgramFiles) { $env:ProgramFiles } elseif ($env:ProgramW6432) { $env:ProgramW6432 } else { $env:ProgramData }
$installRoot = Join-Path $programFiles 'AdBlocker'
$logDir = Join-Path $env:ProgramData 'AdBlocker\logs'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir '..')
$targetDir = Join-Path $repoRoot 'target\release'

function Require-Administrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        throw 'Administrator privileges are required to install the Windows service.'
    }
}

function Ensure-Directories {
    New-Item -ItemType Directory -Force -Path $installRoot | Out-Null
    New-Item -ItemType Directory -Force -Path $logDir | Out-Null
}

function Stop-ExistingService {
    $serviceExe = Join-Path $installRoot 'adblock-service.exe'
    $cliExe = Join-Path $installRoot 'adblockctl.exe'
    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if (-not $service) {
        return
    }

    try {
        Stop-Service -Name $serviceName -Force -ErrorAction Stop
    }
    catch {
        Write-Host "Service stop was already in progress or unavailable: $($_.Exception.Message)"
    }

    $deadline = (Get-Date).AddSeconds(20)
    while ((Get-Service -Name $serviceName -ErrorAction SilentlyContinue) -and (Get-Date) -lt $deadline) {
        Start-Sleep -Milliseconds 250
    }

    for ($attempt = 1; $attempt -le 10; $attempt++) {
        $deleteResult = & sc.exe delete $serviceName 2>&1
        if ($LASTEXITCODE -eq 0) {
            break
        }

        if ($deleteResult -match 'marked for deletion') {
            Start-Sleep -Milliseconds 500
            continue
        }

        break
    }

    foreach ($file in @($serviceExe, $cliExe)) {
        if ($file -and (Test-Path $file)) {
            Remove-Item $file -Force -ErrorAction SilentlyContinue
        }
    }
}

function Build-Binaries {
    Push-Location $repoRoot
    try {
        cargo build --release --workspace --bins
    }
    finally {
        Pop-Location
    }
}

function Ensure-ServiceBinaryIsSigned {
    param(
        [Parameter(Mandatory = $true)]
        [string] $FilePath
    )

    $signature = Get-AuthenticodeSignature -FilePath $FilePath -ErrorAction SilentlyContinue
    if ($signature -and $signature.Status -eq 'Valid') {
        return
    }

    $cert = Get-ChildItem Cert:\LocalMachine\My -CodeSigningCert -ErrorAction SilentlyContinue |
        Sort-Object NotAfter -Descending |
        Select-Object -First 1

    if (-not $cert) {
        $cert = New-SelfSignedCertificate -CertStoreLocation Cert:\LocalMachine\My -Type CodeSigningCert -Subject 'CN=AdBlocker Service Code Signing' -KeyAlgorithm RSA -KeyLength 2048 -HashAlgorithm SHA256 -Provider 'Microsoft Enhanced RSA and AES Cryptographic Provider' -KeyUsage DigitalSignature -FriendlyName 'AdBlocker Service Code Signing'
    }

    $certificatePath = Join-Path $env:TEMP 'AdBlocker-Service-CodeSigning.cer'
    Export-Certificate -Cert $cert -FilePath $certificatePath -Force | Out-Null
    Import-Certificate -FilePath $certificatePath -CertStoreLocation Cert:\LocalMachine\Root -Confirm:$false | Out-Null
    Import-Certificate -FilePath $certificatePath -CertStoreLocation Cert:\LocalMachine\TrustedPublisher -Confirm:$false | Out-Null

    $timestampServer = 'http://timestamp.digicert.com'
    Set-AuthenticodeSignature -FilePath $FilePath -Certificate $cert -TimestampServer $timestampServer -ErrorAction Stop | Out-Null
    Remove-Item $certificatePath -Force -ErrorAction SilentlyContinue
}

function Install-ServiceConfiguration {
    $serviceExe = Join-Path $installRoot 'adblock-service.exe'
    $cliExe = Join-Path $installRoot 'adblockctl.exe'
    $builtService = Join-Path $targetDir 'adblock-service.exe'
    $builtCli = Join-Path $targetDir 'adblockctl.exe'

    if (-not (Test-Path $builtService) -or -not (Test-Path $builtCli)) {
        throw 'Service binaries are missing. Build the workspace before installing the service.'
    }

    foreach ($file in @($serviceExe, $cliExe)) {
        if (Test-Path $file) {
            Remove-Item $file -Force -ErrorAction SilentlyContinue
        }
    }

    Copy-Item $builtService $serviceExe -Force
    Copy-Item $builtCli $cliExe -Force

    Ensure-ServiceBinaryIsSigned -FilePath $serviceExe

    $binPath = '"' + $serviceExe + '" --service'
    Write-Host "Creating service with: New-Service -Name '$serviceName' -BinaryPathName $binPath -DisplayName '$serviceName' -StartupType Automatic"

    New-Service -Name $serviceName -BinaryPathName $binPath -DisplayName $serviceName -StartupType Automatic -ErrorAction Stop | Out-Null

    & sc.exe failure $serviceName reset= 86400 actions= restart/5000/restart/5000/restart/5000
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to configure recovery actions for '$serviceName'."
    }

    & sc.exe description $serviceName 'Windows ad and tracker blocking service'
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to set the service description for '$serviceName'."
    }
}

Require-Administrator
Stop-ExistingService
Ensure-Directories
Build-Binaries
Install-ServiceConfiguration

try {
    Start-Service -Name $serviceName -ErrorAction Stop
}
catch {
    Write-Error "The service could not be started. This commonly happens when Windows App Control / WDAC / AppLocker blocks unsigned service binaries. Try signing the service binary or disabling the policy for this machine. Original error: $($_.Exception.Message)"
    throw
}

Write-Host 'AdBlockService installation complete. The service is configured to start automatically with Windows.'
Write-Host "Installed under: $installRoot"
Write-Host "Logs: $logDir"
