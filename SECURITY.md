# Security Policy

## Supported Versions

Only the current version of PyxisOS is supported for security updates.

| Version | Supported |
| ------- | --------- |
| 0.2.x   | Yes       |
| < 0.2.0 | No        |

## Reporting a Vulnerability

We take the security of PyxisOS seriously, especially regarding:
1. Sandboxing and guest isolation boundaries in `native/aegis/`.
2. Workflow security and execution validation in `automation/`.
3. Agent policy authorization and privilege controls.

If you find a security vulnerability, please do **not** open a public GitHub issue. Instead, report the vulnerability by creating a private security advisory through the repository's security page or email the maintainer at `obstinix@gmail.com`.

We will investigate the issue and attempt to coordinate a resolution before public disclosure.
