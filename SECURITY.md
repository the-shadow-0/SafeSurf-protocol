# Security Policy - SafeSurf Protocol

## Ethical Use
This protocol is for defensive protection ONLY. We strictly forbid use for:
- Accessing or facilitating illegal services.
- Evading lawful surveillance in a manner that facilitates crime.
- Harassment or malicious activities.

## Reporting Vulnerabilities
Please report security issues directly on our [GitHub Issues](https://github.com/the-shadow-0/SafeSurf-protocol/issues). We follow responsible disclosure practices.

## Audit Checklist
- [ ] Constant-time crypto usage.
- [ ] Sensitive memory zeroization (verified via `zeroize`).
- [ ] No plaintext credential logging.
- [ ] Sandbox isolation (per-tab sessions).
