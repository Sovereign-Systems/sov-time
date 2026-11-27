<!--
@version 0.1.0-draft.3
@description Contribution boundaries, source ownership, and verification requirements.
-->

# Contributing to SOV-TIME

## Development boundary

Use the existing project directory and the `main` branch. Do not create a second project root, branch, or worktree as part of the owner's engineering workflow. Keep project-owned generated files, build outputs, caches, temporary files, and evidence within the project root.

Do not publish changes or execute destructive Git operations without authorization.

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
