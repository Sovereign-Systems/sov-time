<!--
@version 0.1.0-draft.11
@description Public project overview, implemented foundation, architecture, model, roadmap, and verification boundaries.
-->

# SOV-TIME

**Deterministic, bounded real-time scheduling analysis and simulation in Rust.**

SOV-TIME is an engineering-focused Rust project for modeling periodic real-time workloads, evaluating schedulability, and producing deterministic scheduling simulations under explicit mathematical and resource bounds. A browser-based WebAssembly interface is planned as a separate hosted layer.

> **Project status:** active development. The low-level domain foundation is now partially implemented and verified, while task modeling, schedulers, schedulability analysis, simulation, and the browser application are still under construction. There is no production release, hardware real-time guarantee, WCET claim, safety certification, or security certification.

The public API is pre-release and may change before the first versioned release. Current documentation distinguishes implemented behavior from planned architecture so that roadmap items are not presented as available features.

## What is implemented now

The current `sov-time-domain` foundation includes:

- `Limit`: a validated strictly positive `u32` finite bound.
- `Instant`: exact abstract `u64` scheduling ticks with checked addition/subtraction and duration calculation.
- `Duration`: exact abstract `u64` ticks with checked addition, subtraction, and multiplication.
- `TimeArithmeticError`: distinct addition-overflow, multiplication-overflow, and subtraction-underflow failures.
- `CapacityState`: bounded acquire/release accounting with atomic failure semantics.
- `Budget`: monotonic bounded work accounting with explicit exhaustion.
- `RelativeDeadline`: a strictly positive wrapper over abstract `Duration`, with typed zero rejection and exact duration ordering.
- public zero-payload domain error enums support `Debug` and equality comparison without adding `Copy`/`Clone` commitments.
- `#![no_std]` domain source with no external crate dependencies in the current foundation manifest.
- typed failure paths with no ordinary `unwrap()`, `expect()`, panic fallback, wrapping arithmetic, saturation, or magic sentinels.

The implemented foundation has been exercised through host formatting, compilation, build, Clippy checks, direct integration-test execution, and direct `wasm32-unknown-unknown` compilation for the exact implemented source increments. These checks do **not** establish a complete workspace release or a global no-heap/WCET guarantee.

## Current foundation example

The snippet below illustrates only the currently implemented public API. It is not a release/install instruction for the still-incomplete six-member workspace.

```rust
use sov_time_domain::{Budget, CapacityState, Duration, Instant, Limit, RelativeDeadline};

fn main() {
    let Ok(capacity_limit) = Limit::new(2) else {
        return;
    };
    let mut capacity = CapacityState::new(capacity_limit);
    if capacity.try_acquire_one().is_err() {
        return;
    }

    let Ok(budget_limit) = Limit::new(3) else {
        return;
    };
    let mut budget = Budget::new(budget_limit);
    if budget.try_consume_one().is_err() {
        return;
    }

    let execution_window = Duration::from_ticks(20);
    let Ok(deadline) = RelativeDeadline::new(execution_window) else {
        return;
    };

    let release = Instant::from_ticks(5);
    let Ok(absolute_deadline) = release.checked_add(deadline.duration()) else {
        return;
    };

    if absolute_deadline.ticks() != 25 {
        return;
    }

    let _resource_state = (capacity.used(), budget.remaining());
}
```

The example uses abstract ticks only. It does not attach seconds, milliseconds, wall-clock time, or hardware timing semantics to the domain values.

## Target model: v0.1

The initial mathematical model is deliberately narrow and explicit:

- one preemptive uniprocessor;
- independent periodic tasks;
- first release at tick `0`;
- abstract nonnegative integer ticks;
- constrained deadlines with `C <= D <= T`;
- no release offsets or jitter;
- no blocking or suspension;
- no precedence constraints;
- no multiprocessor or parallel execution;
- no modeled context-switch overhead;
- no wall-clock, timezone, or physical-unit conversion inside the domain model.

Planned scheduling policies are:

- **EDF** — Earliest Deadline First;
- **RM** — Rate Monotonic;
- **DM** — Deadline Monotonic.

Schedulability analysis and simulation remain distinct: a simulation trace is never treated as a general mathematical proof of schedulability.

## Design principles

SOV-TIME is being built around a small set of non-negotiable engineering properties:

1. **Deterministic semantics** — equivalent explicit inputs must not depend on ambient clock, randomness, filesystem state, network state, or browser state.
2. **Checked arithmetic** — representational failure is explicit and typed; arithmetic does not silently wrap, saturate, clamp, or invent a fallback value.
3. **Bounded work and state** — task counts, proof work, simulation steps, timeline output, and import size are governed by explicit finite limits.
4. **Failure preservation** — invalid input, arithmetic failure, resource exhaustion, unsupported models, inconclusive analysis, and proven unschedulability remain semantically distinct.
5. **Layer separation** — browser/UI effects do not leak into the pure scheduling domain.
6. **Private-by-default APIs** — public surface is introduced only for an identified consumer contract.
7. **Target-specific evidence** — host success is not substituted for browser/WASM, resource, timing, or release evidence.

## Architecture

Dependency direction below is **consumer → dependency**:

```text
crates/application      -> crates/domain
crates/application      -> crates/contracts
crates/browser_interop  -> crates/contracts
crates/web_ui           -> crates/application
crates/web_ui           -> crates/contracts
crates/web_ui           -> crates/browser_interop
apps/web                -> crates/web_ui
```

| Component | Responsibility |
| --- | --- |
| `crates/domain` | Pure domain values, invariants, scheduling, analysis, and bounded simulation |
| `crates/contracts` | Versioned bounded external/import/export representations |
| `crates/application` | Use-case orchestration and conversion between contracts and domain values |
| `crates/browser_interop` | Browser-specific file/input/output effects |
| `crates/web_ui` | Browser presentation, routing, and interaction state |
| `apps/web` | Web application entry/composition |

The browser UI framework has not yet been admitted into the production dependency graph. The initial product direction is a client-side Rust/WASM application with no required backend, database, server functions, or remote API for the core scheduling workflow.

## Resource profile

The current v0.1 working model binds these project-level limits:

| Resource | Bound |
| --- | ---: |
| Tasks | 32 |
| Ready jobs | 32 |
| Active jobs | 32 |
| Timeline segments | 16,384 |
| Analysis work units | 1,048,576 |
| Simulation semantic steps | 65,536 |
| Hosted import tasks | 32 |
| Hosted import bytes | 65,536 |

These are semantic/resource bounds. They are **not** hardware timing or WCET measurements.

## Repository layout

```text
apps/web/                   Browser application entry point
crates/domain/              Pure scheduling-domain foundation
crates/application/         Use-case orchestration
crates/contracts/           Versioned external contracts
crates/browser_interop/     Browser effect boundary
crates/web_ui/              Browser presentation
docs/                       Architecture, model, error and verification records
governance/                 Project-bound engineering/verification records
```

## Current development roadmap

The implementation sequence is dependency-driven:

- [x] validated finite limit foundation;
- [x] abstract checked time primitives;
- [x] bounded capacity and work-budget state;
- [x] positive relative-deadline scalar;
- [ ] stable task identity;
- [ ] validated periodic task parameters and bounded task sets;
- [ ] EDF / RM / DM policy ordering;
- [ ] exact utilization and fixed-priority response-time analysis;
- [ ] constrained-deadline EDF demand-bound analysis;
- [ ] deterministic bounded simulation and timeline generation;
- [ ] application and versioned contract layers;
- [ ] browser interop and Rust/WASM UI;
- [ ] end-to-end release verification and public release.

One current design dependency remains intentionally unresolved: the exact fixed-width backing type for `TaskId` has not yet been bound. The implementation does not guess that contract.

## Toolchain and verification

- Rust: **1.99.1**
- Edition: **2024**
- Primary browser target: `wasm32-unknown-unknown`
- Workspace lint policy forbids unsafe code and denies `unwrap`, `expect`, `panic`, `todo`, `unimplemented`, unchecked arithmetic-side-effect patterns, and unchecked indexing/slicing patterns in governed first-party code.

The complete six-member workspace is **not yet release-qualified** because several higher layers remain scaffold/in-progress. Build instructions will be promoted here when the complete workspace graph is compile-ready and its target/profile gates are satisfied.

## Documentation

- [API guide](docs/API_GUIDE.md)
- [Testing guide](docs/TESTING.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Design record](docs/DESIGN.md)
- [Mathematical model](docs/MATHEMATICAL_MODEL.md)
- [Memory and resource model](docs/MEMORY_MODEL.md)
- [Error model](docs/ERROR_MODEL.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

## Important limitations

SOV-TIME models and analyzes scheduling semantics. It does not measure execution time on your hardware, prove WCET from source code, certify a system as safe, or establish that an arbitrary real system satisfies the model assumptions.

Any future production claim must be tied to the exact source revision, target, dependency graph, configuration, mathematical assumptions, and verification evidence that support it.

## License

Licensed under the [MIT License](LICENSE).
