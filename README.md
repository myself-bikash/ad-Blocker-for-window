# Ad Blocker for Windows

A lightweight Windows-native ad blocker built around a persistent background service and a simple CLI control tool.

Repository: https://github.com/myself-bikash/ad-Blocker-for-window

This project is designed as a practical, service-first MVP for maximum practical ad and tracker blocking on Windows without claiming universal coverage.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Important Note](#important-note)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Master Command](#master-command)
- [Build and Test](#build-and-test)
- [Install as Administrator](#install-as-administrator)
- [Stale Service Cleanup](#stale-service-cleanup)
- [CLI Commands](#cli-commands)
- [Uninstall](#uninstall)
- [GitHub First Push Commands](#github-first-push-commands)
- [Current Status](#current-status)
- [Recommended Next Step](#recommended-next-step)

## Overview

- Runs as a Windows background service
- Stores state locally on the machine
- Supports enable, disable, status, and restart actions through the CLI
- Uses fail-open safety behavior for network stability
- Focuses on common ad-serving and tracking domains rather than browser-only filtering

## Features

- Windows service installation and lifecycle control
- CLI control utility for status and service actions
- Default domain rules for ad and tracker blocking
- Wildcard and subdomain coverage for practical blocking
- Safe allowlist behavior for trusted platform domains
- Local persistence for service state and runtime configuration

## Architecture

- `crates/core` — rule evaluation and precedence logic
- `crates/rules` — default allow/block rules and platform-specific rules
- `crates/config` — service configuration and defaults
- `crates/dns` — DNS filtering model and cache behavior
- `crates/network` — health checks and fail-open safety
- `crates/wfp` — Windows Filtering Platform integration layer
- `crates/service` — Windows service runtime state and lifecycle
- `crates/cli` — CLI control commands
- `scripts` — PowerShell install and uninstall helpers

## Important Note

This is not a guarantee of blocking every ad on every app or website on Windows. The goal is practical, system-wide filtering with a safe background-service approach.

## Prerequisites

Before installing, make sure the target Windows machine has:

- Rust toolchain installed
- Visual Studio C++ Build Tools / MSVC toolchain
- Windows SDK libraries required for Rust linking
- Administrator PowerShell access

## Quick Start

```powershell
git clone https://github.com/myself-bikash/ad-Blocker-for-window.git
cd ad-Blocker-for-window
.\scripts\install.ps1
```

> If you are already inside the repository folder, skip the `cd` step and run the installer directly.
> On Windows 10 and 11, the most common cause of a service failing to start is Windows App Control / WDAC / AppLocker blocking an unsigned executable. This is a Windows security policy check, not proof that the project is malicious. The installer attempts to create and trust a local code-signing certificate and signs the service before installation.

> The installer script must be run from an Administrator PowerShell window because it creates and configures a Windows service.

## Master Command

Run this single command in PowerShell to clone the project, open an elevated PowerShell window, build the release binaries, install the service, and start it:

```powershell
$repo = Join-Path $HOME 'ad-Blocker-for-window'; git clone 'https://github.com/myself-bikash/ad-Blocker-for-window.git' $repo; Set-Location $repo; Start-Process powershell.exe -Verb RunAs -Wait -ArgumentList '-NoProfile','-ExecutionPolicy','Bypass','-File',"$repo\scripts\install.ps1"
```

The elevated window may ask for Administrator approval. After it finishes, verify the service with:

```powershell
Get-Service AdBlockService
```

## Build and Test

From the project root:

```powershell
cargo build --release --workspace --bins
cargo test -p adblock-service --lib -- --nocapture
cargo test -p adblock-core -- --nocapture
cargo test -p adblock-rules --lib -- --nocapture
```

> This project installs a Windows background service that can trigger Microsoft Defender or Windows App Control warnings. The service should be signed before distribution and installed only by a user who understands the system-level network filtering behavior.

## Windows Security Note

Windows may flag the service as suspicious because it is a background Windows service that modifies network-related behavior. This is usually caused by Windows Defender SmartScreen, App Control for Business, WDAC, or AppLocker policies. The project is not inherently malicious, but because it installs a local service that filters or intercepts network activity, Windows treats it more strictly than a normal app.

The correct production-safe approach is to sign the service binary with a valid Authenticode certificate, install it in a trusted location such as `C:\Program Files`, and run the installer as Administrator only after the user understands that the service changes system-level network behavior.

## Install as Administrator

```powershell
cd ad-Blocker-for-window
.\scripts\install.ps1
```

The script will:

- check Administrator privileges
- stop and clean any stale AdBlockService
- build the binaries
- create a local code-signing certificate if needed
- sign the service executable so Windows 10/11 App Control allows it
- copy them into `%ProgramFiles%\AdBlocker`
- create the Windows service
- start the service automatically

For a signed release build, run:

```powershell
cargo build --release --workspace --bins
.\scripts\sign-release.ps1
.\scripts\install.ps1
```

> The signing helper is intended for production-safe Windows distribution. In some environments, the OS will still block unsigned build artifacts or service binaries because they modify system-level network behavior.

## Stale Service Cleanup

If a previous install left a stale service behind, run:

```powershell
Stop-Service AdBlockService -ErrorAction SilentlyContinue
sc.exe delete AdBlockService -ErrorAction SilentlyContinue
.\scripts\install.ps1
```

## CLI Commands

After installation, use the CLI:

```powershell
.\target\debug\adblockctl.exe status
.\target\debug\adblockctl.exe enable
.\target\debug\adblockctl.exe disable
.\target\debug\adblockctl.exe restart
```

Check the Windows service directly:

```powershell
Get-Service AdBlockService
```

## Uninstall

```powershell
.\scripts\uninstall.ps1
```

## GitHub First Push Commands

If you want to push this repo manually from a fresh local folder:

```bash
git init
git add .
git commit -m "initial project commit"
git branch -M main
git remote add origin https://github.com/myself-bikash/ad-Blocker-for-window.git
git push -u origin main
```

If you only want to initialize with the README as the first commit:

```bash
echo "# ad-Blocker-for-window" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/myself-bikash/ad-Blocker-for-window.git
git push -u origin main
```

## Current Status

This project is currently in a verified Windows service-first MVP state:

- workspace builds successfully
- service installs and runs
- CLI controls work as expected
- default rule engine and wildcard matching are passing tests
- runtime rule refresh and DNS enforcement are in place

## Recommended Next Step

For stronger real-world filtering beyond this MVP, the next improvement would be deeper live DNS/WFP enforcement and more production-grade rule sources.
