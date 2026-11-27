<!--
@version 0.1.0-draft.2
@description Security reporting, trust boundaries, and supported-release policy.
-->

# Security policy

## Supported versions

SOV-TIME is in early development and has no supported production release. No security audit or certification is claimed.

## Reporting vulnerabilities

Do not publish exploit details, sensitive inputs, credentials, or private information in a public issue.

Use the repository's private vulnerability reporting facility if it is enabled. If it is unavailable, contact the repository owner through a verified private channel before sharing sensitive reproduction information. A dedicated reporting address and response commitments have not yet been established.

A useful report identifies the affected revision, target, configuration, expected behavior, observed behavior, impact, and a minimal reproduction using non-sensitive data.

## Security boundaries

The implemented domain foundation uses explicit validated values, checked arithmetic, typed failures, and bounded resource-accounting primitives. Future externally supplied task parameters and scenarios remain untrusted inputs; their validation, capacity, analysis-termination, and deterministic failure requirements must be enforced before those higher-level workflows are advertised as implemented.

Browser-specific effects belong outside the strict domain foundation. Imported data must be validated before it reaches domain computations. Browser storage, file import, and export are separate trust boundaries. No source-text statement in this policy establishes a global no-heap, WCET, or target-security guarantee.

## Assurance limitations

A deterministic scheduling simulation is not a real-time execution guarantee. Host tests and browser tests do not establish hardware timing properties. Safety-critical or mission-critical use requires separate suitability analysis and target-specific verification.

Security fixes must be verified against the affected revision, including applicable regression and negative tests. No unsupported security assurance claim may be made.
