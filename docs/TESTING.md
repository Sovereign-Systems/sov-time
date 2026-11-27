<!--
@version 0.1.0-draft.1
@description Public testing and verification guide for the current SOV-TIME development state.
-->

# SOV-TIME Testing Guide

SOV-TIME uses verification evidence that is scoped to the exact component, source snapshot, target, and claim being evaluated. A passing check in one context is not treated as proof for a different target or for the complete repository.

The project is still under staged construction. This document describes the testing posture that exists today without presenting the incomplete six-member workspace as a released build.

## Toolchain

The repository selects Rust `1.99.1` in `rust-toolchain.toml`.

A contribution should use the repository-selected toolchain unless a separately reviewed change intentionally updates that binding.

## Current public test surface

The implemented domain foundation currently has public integration coverage for:

- positive `Limit` construction and zero rejection;
- minimum and maximum `u32` limit values;
- zero and maximum abstract time values;
- checked time addition, subtraction, multiplication, and duration-since behavior;
- exact overflow and underflow error classification;
- unsigned tick ordering and repeatability;
- capacity acquisition, exhaustion, release, release underflow, and state preservation on failure;
- monotonic budget consumption, exhaustion, deterministic repetition, and state preservation on failure;
- `RelativeDeadline` zero rejection, one-tick acceptance, maximum-tick acceptance, and ordering;
- public error `Debug`/equality trait availability;
- preservation of distinct error-variant identity.

The current tracked integration test root is:

`crates/domain/tests/integration_tests.rs`

The higher-level scheduler, analysis, simulation, browser, and web application test surfaces are not yet implemented public test suites.

## Verification layers

For an applicable Rust change, verification is separated into distinct layers.

### Formatting

Source formatting is checked independently. A formatting pass establishes formatting conformity only; it does not establish compilation, behavior, target closure, or release readiness.

### Compilation

Compiler checks establish that the selected source and configuration compile in the checked context. Compilation alone does not establish that required tests executed or that another target also builds.

### Clippy

Applicable Clippy policy is checked separately. The current workspace policy denies ordinary uses of `unwrap`, `expect`, `panic`, `todo`, `unimplemented`, `dbg!`, unchecked arithmetic side effects, and dynamic indexing/slicing through the configured lint set.

A lint pass is not a substitute for behavioral tests.

### Behavioral tests

Tests are expected to assert exact observable behavior, including negative paths and boundary conditions where the contract requires them.

For mutable resource accounting, failure-path tests must verify that failed transitions preserve the observable state.

For checked arithmetic, tests must distinguish the applicable overflow or underflow cause rather than accepting any generic failure.

### Target-specific verification

Host success is not automatically target success.

The implemented domain increments have also been checked against `wasm32-unknown-unknown` in the owner-controlled engineering workflow. Those checks are scoped to the exact verified source snapshots and do not establish a permanent global target guarantee for future revisions.

### Structural and claim checks

Verification also includes source-structure and claim review where applicable, such as:

- no `mod.rs` introduction;
- no ordinary `unsafe` introduction into the current foundation;
- no hidden panic-shortcut replacement for typed failure;
- no accidental expansion of public API traits or conversions;
- no claim of no-heap, WCET, real-time execution, or release readiness from source syntax alone.

## Test-driven behavioral changes

Behavioral changes should begin with a discriminating test or equivalent oracle whenever feasible.

A valid correction cycle should show:

1. the intended requirement and observable failure;
2. a failing test or compile-time oracle for the missing or incorrect behavior;
3. the smallest cohesive implementation change;
4. a passing re-run of the relevant checks;
5. regression coverage for the corrected behavior.

A test that accepts both success and failure does not establish the required behavior.

## Current public reproducibility limitation

The complete root Cargo workspace is not yet a supported public verification entry point because higher-layer members remain under staged construction, including unpopulated web UI and web entry manifests.

Therefore this document does not advertise a root-level `cargo test`, `cargo build`, CI badge, or release-verification command as a currently supported public contract.

This limitation is about the incomplete repository-wide build surface. It does not convert unimplemented higher-layer scaffolds into failures of the already implemented domain semantics.

## Contribution evidence

For a pull request or other contribution, report:

- the exact files and behavior changed;
- the Rust toolchain used;
- the target and relevant feature set;
- formatter/compiler/lint/test commands actually executed;
- test counts or otherwise sufficient evidence that required cases ran;
- negative and boundary cases exercised;
- any skipped, blocked, or not-applicable verification;
- any claim that remains intentionally unproven.

Do not report a check as passed unless it was actually executed and its result was inspected.

## What a PASS does not mean

No individual formatting, compiler, lint, test, host, or WASM result establishes all of the following:

- complete repository conformance;
- production readiness;
- a versioned release;
- hardware real-time behavior;
- WCET;
- safety certification;
- security certification;
- a global no-heap proof.

Those require their own applicable evidence.

## Related documentation

- [API guide](API_GUIDE.md)
- [Contributing](../CONTRIBUTING.md)
- [Project overview](../README.md)
- [Architecture](ARCHITECTURE.md)
- [Mathematical model](MATHEMATICAL_MODEL.md)
- [Memory and resource model](MEMORY_MODEL.md)
- [Error model](ERROR_MODEL.md)
- [Security policy](../SECURITY.md)
