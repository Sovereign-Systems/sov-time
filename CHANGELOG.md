<!--
@version 0.1.0-draft.5
@description Chronological record of verified project changes and release status.
-->

# Changelog

This file records project changes. Entries under Unreleased describe development state, not a published release.

## Unreleased

### Added

- Initial project directory and public GitHub repository.
- Single-branch Git initialization using `main`.
- Workspace directory scaffold separating the domain, application, contracts, browser interoperability, web interface, and web entry point.
- Initial project overview, contribution requirements, and security reporting guidance.
- Exact Rust 1.99.1 toolchain selection in the local root configuration; complete component, target, and build verification remains pending.
- Verified Git ignore behavior for project-local build outputs, local evidence, private source bindings, and environment files. Cargo.lock remains eligible for tracking.
- Rust 2024 parsing and style edition selection in `rustfmt.toml`; TOML syntax verified.
- Explicit Clippy configuration excluding `unwrap` and `expect` exemptions; exact content and TOML syntax verified. Effective lint enforcement remains pending.

- Persistent implementation backlog in `plan.md` with task dependencies, acceptance criteria, and explicit resume checkpoints.

- MIT License selected under delegated owner authority and written to `LICENSE`; manifest metadata reconciliation remains pending.

### Pending

- Complete toolchain reconciliation and Cargo workspace configuration.
- Implementation and verification of the domain engine and hosted web application.
- License metadata reconciliation and release preparation.

No versioned software release has been published.
