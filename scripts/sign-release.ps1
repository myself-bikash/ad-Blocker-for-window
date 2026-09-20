$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$releaseDir = Join-Path $repoRoot 'target\release'
$filesToSign = @(
    'adblock-service.exe',
    'adblockctl.exe'
)

function Ensure-CodeSigningCertificate {
    $cert = Get-ChildItem Cert:\LocalMachine\My -CodeSigningCert -ErrorAction SilentlyContinue |
        Sort-Object NotAfter -Descending |
        Select-Object -First 1

    if (-not $cert) {
        Write-Host 'Creating a local code-signing certificate for release signing...'
        $cert = New-SelfSignedCertificate -CertStoreLocation Cert:\LocalMachine\My -Type CodeSigningCert `
            -Subject 'CN=AdBlocker Service Code Signing' `
            -KeyAlgorithm RSA `
            -KeyLength 2048 `
            -HashAlgorithm SHA256 `
            -Provider 'Microsoft Enhanced RSA and AES Cryptographic Provider' `
            -KeyUsage DigitalSignature `
            -FriendlyName 'AdBlocker Service Code Signing'
    }

    $certificatePath = Join-Path $env:TEMP 'AdBlocker-Service-CodeSigning.cer'
    try {
        Export-Certificate -Cert $cert -FilePath $certificatePath -Force | Out-Null
        try {
            Import-Certificate -FilePath $certificatePath -CertStoreLocation Cert:\LocalMachine\Root -Confirm:$false | Out-Null
        }
        catch {
            Write-Warning "Could not add the certificate to the Root store: $($_.Exception.Message)"
        }

        try {
            Import-Certificate -FilePath $certificatePath -CertStoreLocation Cert:\LocalMachine\TrustedPublisher -Confirm:$false | Out-Null
        }
        catch {
            Write-Warning "Could not add the certificate to the TrustedPublisher store: $($_.Exception.Message)"
        }
    }
    finally {
        Remove-Item $certificatePath -Force -ErrorAction SilentlyContinue
    }

    return $cert
}

function Sign-ReleaseBinary {
    param(
        [Parameter(Mandatory = $true)]
        [string] $FilePath,

        [Parameter(Mandatory = $true)]
        [System.Security.Cryptography.X509Certificates.X509Certificate2] $Certificate
    )

    $signature = Get-AuthenticodeSignature -FilePath $FilePath -ErrorAction SilentlyContinue
    if ($signature -and $signature.Status -eq 'Valid') {
        Write-Host "Already signed: $FilePath"
        return
    }

    Set-AuthenticodeSignature -FilePath $FilePath -Certificate $Certificate -TimestampServer 'http://timestamp.digicert.com' -ErrorAction Stop | Out-Null
    Write-Host "Signed: $FilePath"
}

if (-not (Test-Path $releaseDir)) {
    Write-Error "Release artifacts were not found at '$releaseDir'. Run the following first: cargo build --release --workspace --bins"
    exit 1
}

$cert = Ensure-CodeSigningCertificate

$missingFiles = @()
foreach ($file in $filesToSign) {
    $filePath = Join-Path $releaseDir $file
    if (-not (Test-Path $filePath)) {
        $missingFiles += $filePath
        continue
    }

    Sign-ReleaseBinary -FilePath $filePath -Certificate $cert
}

if ($missingFiles.Count -gt 0) {
    Write-Error "Missing release binary files: $($missingFiles -join ', ')"
    exit 1
}

Write-Host "Release signing complete."
Write-Host "Signed files are under: $releaseDir"
