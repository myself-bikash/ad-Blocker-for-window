# Security Policy

This project is designed to run with elevated Windows privileges and therefore must treat network configuration and filter-list data as untrusted input.

## Key principles

- Fail open instead of breaking connectivity
- Validate downloaded filter lists before activation
- Use local-only processing without telemetry
- Never silently install certificates or intercept HTTPS without explicit user action
- Require secure local IPC for management commands

## Reporting

Please report security issues privately to the maintainer or repository owner.
