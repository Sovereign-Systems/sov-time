<!--
@version 0.1.0-draft.1
@description Public reproducibility model for SOV-TIME semantics, verification evidence, and current build limitations.
-->

# SOV-TIME Reproducibility

SOV-TIME distinguishes deterministic domain semantics from reproducible software builds.

The current domain foundation is designed for explicit-input deterministic behavior. The complete repository is not yet a reproducible release build: the workspace is still under staged construction, the final dependency graphs are unresolved, and no `Cargo.lock` exists yet.

## Reproducibility categories

### Semantic reproducibility

For an implemented deterministic domain operation, identical valid inputs under the same declared model are expected to produce the same typed result.

The current foundation supports this through:

- abstract integral ticks rather than ambient wall-clock reads;
- explicit validated values;
- checked arithmetic;
- explicit capacity and work-budget state;
- typed failure causes;
- no browser, filesystem, network, environment, randomness, telemetry, or persistence input in domain operations.

Semantic repeatability does not imply hardware timing repeatability.

### Verification reproducibility

A verification result is meaningful only with its relevant inputs identified.

For material Rust checks, record as applicable:

- repository revision;
- source snapshot or file identities;
- Rust toolchain;
- target;
- feature set;
- build/test profile;
- command or checker class;
- test/verification scope;
- result;
- known limitations.

A result from one source snapshot or target must not be silently reused for a materially different one.

### Build reproducibility

Complete build reproducibility requires the complete build input set to be frozen and available.

That state is not established yet for SOV-TIME.

Current blockers include:

- no `Cargo.lock`;
- unpopulated web UI and web entry manifests;
- unresolved final framework selection;
- unresolved complete dependency and feature graphs;
- unresolved final production build/link configuration;
- incomplete browser-runtime qualification.

Accordingly, SOV-TIME does not currently claim bit-for-bit reproducible builds or a fully frozen release source set.

## Bound Rust toolchain

The repository selects Rust `1.99.1` through `rust-toolchain.toml`.

The current build-matrix record also binds the verified toolchain family used by the owner-controlled engineering workflow, including the host and `wasm32-unknown-unknown` target contexts.

A toolchain version alone is not a complete reproducibility record. Target installation, dependency resolution, source identity, features, build flags, linker inputs, and environment may also affect an artifact.

## Project-local build artifacts

The repository configures Cargo build output under:

`.local/target`

This keeps generated compiler artifacts inside the project-local ignored area in the canonical engineering workspace.

The target directory is not source authority and is not committed as a reproducible artifact.

## Deterministic time model

The current time primitives use abstract `u64` ticks.

A tick has no implicit relationship to:

- wall-clock time;
- timezone time;
- seconds or milliseconds;
- CPU cycles;
- browser timers;
- operating-system clock sources.

This prevents hidden time-source behavior from entering the current domain semantics.

Any future mapping from ticks to a physical or runtime time unit must be explicit in the consuming boundary.

## Deterministic failure behavior

The current foundation uses typed results for represented failure paths.

Examples include:

- zero-limit rejection;
- addition overflow;
- multiplication overflow;
- subtraction underflow;
- capacity exhaustion;
- release underflow;
- budget exhaustion;
- zero relative deadline.

For mutable resource accounting, a rejected transition preserves the prior observable state. This prevents a repeated failure from depending on an undocumented partial mutation.

## Resource bounds and reproducibility

Explicit bounds improve repeatability by preventing hidden unbounded work from being treated as normal behavior.

The v0.1 design binds limits for task count, analysis work, simulation steps, timeline capacity, and hosted import size. Only the currently implemented limit/capacity/budget primitives are executable public controls today.

A numeric work bound is not a CPU-time or WCET guarantee.

## Host versus WASM evidence

Host and `wasm32-unknown-unknown` results are separate evidence classes.

A host PASS does not prove:

- WASM compilation;
- browser behavior;
- browser performance;
- stack behavior on another target;
- hardware real-time behavior.

The implemented domain increments have been checked in both host-oriented verification and direct WASM compilation contexts for exact source snapshots. Future revisions must re-establish any affected claim.

## Public examples

The foundation walkthrough in the [API guide](API_GUIDE.md) is kept identical to the walkthrough in the project [README](../README.md).

The current walkthrough has been compiler-, Clippy-, and build-checked against the matching implemented domain source snapshot.

That example evidence proves only the example and its referenced API surface. It is not a complete workspace qualification.

## Dependency reproducibility

The current `sov-time-domain` manifest has no external crate dependencies.

The complete workspace does not yet have a final external dependency graph. When dependencies are admitted, reproducibility review must include:

- exact dependency resolution;
- lockfile state;
- feature resolution;
- target-specific dependency reachability;
- build scripts and proc macros;
- native inputs where applicable;
- source provenance.

No future dependency should be treated as reproducibly resolved merely because a version range appears in a manifest.

## Current public limitations

SOV-TIME currently does not claim:

- a reproducible release build;
- a frozen complete dependency graph;
- a release lockfile;
- bit-for-bit artifact identity across machines;
- deterministic browser rendering;
- deterministic hardware execution time;
- WCET;
- a complete browser support matrix.

These claims require their own completed build/runtime definitions and evidence.

## Reporting a reproduction

When reporting a build, test, or behavior reproduction, include:

- the exact commit;
- operating target or runtime context;
- Rust toolchain version;
- relevant features/configuration;
- input values or a minimal non-sensitive fixture;
- exact observed result;
- exact expected result;
- commands or steps actually executed.

Do not substitute a later revision's success for the revision where the behavior was observed.

## Related documentation

- [API guide](API_GUIDE.md)
- [Testing guide](TESTING.md)
- [Security model](SECURITY_MODEL.md)
- [Architecture](ARCHITECTURE.md)
- [Design record](DESIGN.md)
- [Mathematical model](MATHEMATICAL_MODEL.md)
- [Memory and resource model](MEMORY_MODEL.md)
- [Project overview](../README.md)
