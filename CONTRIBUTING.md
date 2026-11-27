<!--
@version 0.1.0-draft.4
@description Contribution boundaries, source ownership, and verification requirements.
-->

# Contributing to SOV-TIME

## Contribution workflow

The canonical owner-controlled engineering workspace uses one project root and the `main` branch only. Maintainers working in that workspace must not create a second project root, local feature branch, or worktree, and must not publish or perform destructive Git operations without the applicable authorization.

External contributors are not expected to reproduce the owner's local workspace policy. Use a normal GitHub contribution path: work in your own fork or GitHub-managed contribution branch and open a pull request targeting this repository's `main` branch. Keep each pull request narrowly scoped and explain the requirement, behavior change, verification performed, and any unresolved limitation.

Do not commit generated build outputs, caches, local evidence, environment files, credentials, or private source-binding records.

## Source changes

Before modifying a file, identify its responsibility, primary source class, applicable runtime profile, build contexts, invariants, public API impact, and verification obligations.

First-party Rust files use the source identity marker:

```rust
// sovereign-rust/v1 {"id":"example.identity","purpose":"Describe the owned responsibility."}
```

The marker identifies the file and its purpose. It does not replace module documentation, public API documentation, or the corresponding structure registry entry.

Use `name.rs` and `name/child.rs` for filesystem-backed modules with children. Do not introduce `mod.rs`. Do not use `unwrap()`, `expect()`, or equivalent panic-on-failure shortcuts as ordinary first-party implementation or test logic. An isolated intentionally invalid rejection fixture may contain a prohibited construct only when its purpose is to prove that the configured checker rejects that construct, the expected rejection is asserted, and the fixture cannot enter production reachability.

Keep domain logic independent of hosted application and browser dependencies. Make capacity, arithmetic, resource, error, and termination constraints explicit. Do not introduce dependencies solely for convenience without evaluating their applicable build and runtime graphs.

## Verification

For each behavioral change:

1. Identify the requirement and expected failure behavior.
2. Establish the applicable source contract and verification plan before editing.
3. Implement the smallest cohesive change with explicit error handling.
4. Verify source structure, ownership, and affected invariants.
5. Run the applicable formatter, compiler, lints, documentation checks, and tests using the project-bound toolchain.
6. Exercise negative cases and resource limits where applicable.
7. Record exact commands, configurations, outcomes, and unresolved limitations.

Host tests do not prove target-specific properties. A passing build does not establish complete conformance. Do not report a check as passed unless it was executed and its result verified.

## Publication

A contribution is not a release. The project currently has no released implementation. The selected project license is MIT; see [LICENSE](LICENSE) for its terms. Publication and release decisions remain separate from local development.
