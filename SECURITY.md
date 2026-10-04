# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 1.0.x   | ✅        |

## Reporting a Vulnerability

Please do not open a public issue for security vulnerabilities. Instead, use
GitHub's private vulnerability reporting: open the repository's **Security**
tab and choose **Report a vulnerability**. You can expect an initial response
within 7 days.

## Scope Notes

- `toll-optimizer` is fully offline. It makes no network requests, collects no
  telemetry, and never transmits your statement data anywhere.
- Prebuilt binaries are published through GitHub Releases with a `SHA256SUMS.txt`
  file. Verify your download before running it (see README).
- The 407 ETR rate tables are compiled into the binary. If they are out of date,
  the tool prints a warning to stderr; that is a data-freshness issue, not a
  security issue.
