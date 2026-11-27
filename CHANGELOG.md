<!--
@version 0.1.0-draft.19
@description Chronological record of verified project changes and release status.
-->

# Changelog

This file records project changes. Entries under Unreleased describe development state, not a published release.

## Unreleased

### Added

- Initial project directory and public GitHub repository.
- Initial reviewed public scaffold published to the `main` branch.
- Public README no longer links to unpublished internal development records.
- Single-branch Git initialization using `main`.
- Workspace directory scaffold separating the domain, application, contracts, browser interoperability, web interface, and web entry point.
- Published root virtual Cargo workspace plus project-local Cargo target output configuration under `.local/target`.
- Public project overview now treats the browser UI framework as pending engineering selection instead of an admitted dependency, and avoids assuming an arbitrary stored task-priority field.
- Initial project overview, contribution requirements, and security reporting guidance.
- Exact Rust 1.99.1 toolchain selection in the root configuration, with project-bound verification of rustc, Cargo, rustfmt, Clippy, the host target, and `wasm32-unknown-unknown`; complete member/workspace build verification remains pending.
- Verified Git ignore behavior for project-local build outputs, local evidence, private source bindings, environment files, and the unpublished internal `plan.md`. Cargo.lock and Cargo manifests remain eligible for tracking.
- Rust 2024 parsing and style edition selection in `rustfmt.toml`; TOML syntax verified.
- Explicit Clippy configuration excluding `unwrap` and `expect` exemptions; isolated negative fixtures verified effective `unwrap_used` and `expect_used` rejection, followed by a clean Clippy pass after source restoration.
- Contribution guidance now distinguishes prohibited ordinary panic-shortcut usage from isolated intentionally invalid rejection fixtures used solely to prove checker enforcement.
- Implemented the initial `sov-time-domain` foundation: positive finite limits, exact abstract `Instant`/`Duration` ticks with checked arithmetic, bounded capacity accounting, and monotonic work budgets.
- Added typed boundary failures for zero limits, time arithmetic overflow/underflow, capacity exhaustion/release underflow, and budget exhaustion.
- Added a validated strictly positive `RelativeDeadline` scalar over abstract duration ticks, including zero rejection and boundary/order tests.
- Bound and implemented `TaskId` as an exact fixed-width `u32` semantic identity value with no reserved sentinel, full `0..=u32::MAX` representability, exact equality, and a deliberately minimal public trait/API surface.
- Added validated immutable `TaskParameters` for the v0.1 periodic constrained-deadline relation and a borrowed bounded `TaskSet` with distinct caller-limit, owner-capacity, checked count-conversion, and duplicate-identity failures; the owner task maximum is 32 and duplicate scanning is bounded to at most 496 comparisons.
- Verified the implemented domain increments with host formatting/check/build/Clippy, direct integration-test execution, and direct `wasm32-unknown-unknown` compilation plus allocator/std reachability scanning for the exact verified source snapshots.
- Clarified the public contribution workflow so external fork/pull-request participation is distinct from the canonical owner's single-root, `main`-only local engineering policy.
- Updated the public security policy to describe the implemented bounded domain foundation accurately without implying an established global no-heap, WCET, or target-security guarantee.
- Added `Debug`, `Eq`, and `PartialEq` to the existing public domain error enums while deliberately avoiding `Copy`/`Clone`, new failure variants, or conversion APIs.
- Added a compiler- and Clippy-verified README example for the currently implemented domain foundation, plus explicit pre-release API stability and documentation links.
- Added a public API guide for the currently implemented domain foundation, with exact type/error coverage, failure semantics, a verified foundation walkthrough, explicit future-API exclusions, and the current staged-workspace limitation.
- Added a public testing guide that documents the current domain test surface, evidence layers, target-specific boundaries, TDD expectations, contribution evidence, and the current public reproducibility limitation without implying a release pipeline.
- Added a public security model that separates implemented domain controls from future trust boundaries, documents failure/resource/dependency posture, and makes current non-surfaces and assurance limitations explicit.
- Added a reproducibility guide that distinguishes deterministic domain semantics and scoped verification evidence from the not-yet-established complete reproducible build, including the current lockfile/workspace/dependency limitations.

- MIT License selected under delegated owner authority and written to `LICENSE`; the published root Cargo workspace metadata declares `license = "MIT"` consistently.

### Pending

- Populate and review member manifests, generate `Cargo.lock` with Cargo, and verify the complete host/WASM dependency, feature, and build graphs.
- Complete scheduler policy ordering, schedulability analysis, bounded simulation, application/contracts layers, and the hosted web application.
- Release preparation after implementation, complete verification, and applicable lifecycle gates.

No versioned software release has been published.
