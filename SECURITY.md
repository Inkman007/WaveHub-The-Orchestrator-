# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Please report security issues by emailing **security@wavehub.example** with:

- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested mitigations

You will receive an acknowledgement within 48 hours and a resolution timeline within 7 days.

## Scope

- Smart contract logic in `contracts/wavehub/src/`
- Authorization and access-control paths
- Integer overflow / underflow in pool or timestamp arithmetic

## Out of scope

- Issues in third-party dependencies (report upstream)
- Theoretical attacks with no practical exploit path
