<!--
@version 0.1.0-draft.2
@description Public guide to the currently implemented SOV-TIME domain foundation API.
-->

# SOV-TIME API Guide

SOV-TIME is under active development. This guide documents only the public API that is implemented in the current `sov-time-domain` foundation. It does not document planned scheduler, schedulability-analysis, simulation, browser, or serialization APIs as if they already existed.

The API is pre-release and may change before the first versioned release.

The complete six-member Cargo workspace is also still under staged construction. In particular, the web UI and web entry manifests are not yet populated, so this guide intentionally does not present a root-workspace build, `cargo install`, or crates.io installation command as a supported public workflow. The implemented domain foundation is documented here without implying that the complete repository is release-buildable.

## Current crate surface

The current public domain facade exports:

```text
Limit
LimitError

Instant
Duration
TimeArithmeticError

CapacityState
CapacityError

Budget
BudgetError

TaskId

RelativeDeadline
DeadlineError
```

The domain crate is `#![no_std]` and currently has no external crate dependencies. Those source properties do not by themselves establish a global no-heap, WCET, hardware real-time, or release guarantee.

## Abstract ticks

`Instant`, `Duration`, and `RelativeDeadline` use abstract integral ticks.

A tick has no implicit mapping to seconds, milliseconds, CPU cycles, wall-clock time, timezone time, or another physical unit. Any such mapping belongs to a separately defined consuming context.

### Instant

`Instant` stores an exact `u64` tick value. Every `u64` value is valid.

Available operations:

| API | Result |
|---|---|
| `Instant::ZERO` | tick zero |
| `Instant::from_ticks(u64)` | constructs an exact abstract instant |
| `ticks()` | returns the exact stored tick value |
| `checked_add(Duration)` | returns a later instant or `AdditionOverflow` |
| `checked_sub(Duration)` | returns an earlier instant or `SubtractionUnderflow` |
| `checked_duration_since(Instant)` | returns elapsed duration or `SubtractionUnderflow` |

`Instant` supports `Copy`, `Clone`, `Debug`, equality, and total ordering.

### Duration

`Duration` stores an exact `u64` tick count. Zero is a valid generic duration.

Available operations:

| API | Result |
|---|---|
| `Duration::ZERO` | zero ticks |
| `Duration::from_ticks(u64)` | constructs an exact abstract duration |
| `ticks()` | returns the exact stored tick value |
| `checked_add(Duration)` | exact sum or `AdditionOverflow` |
| `checked_sub(Duration)` | exact difference or `SubtractionUnderflow` |
| `checked_mul(u64)` | exact scalar product or `MultiplicationOverflow` |

`Duration` supports `Copy`, `Clone`, `Debug`, equality, and total ordering.

Checked arithmetic never silently wraps or saturates.

## Positive finite limits

`Limit` represents a strictly positive finite `u32` bound.

Use `Limit::new(raw)` to construct a bound. Positive values are accepted, `Limit::new(0)` returns `LimitError::Zero`, and the exact stored value is available through `get()`. A `Limit` is a reusable bound value; it does not allocate storage or acquire capacity by itself.

## Task identity

`TaskId` is the stable semantic identity value for a periodic task. Its owner-bound v0.1 backing type is fixed-width `u32`.

Use `TaskId::from_raw(raw)` to construct an identity and `get()` to recover the exact stored value. Every `u32` value is representable, including `0` and `u32::MAX`; there is no reserved sentinel value.

`TaskId` supports `Copy`, `Clone`, `Debug`, `Eq`, and `PartialEq`. It deliberately does not currently expose ordering, hashing, default construction, parsing, blanket conversion, or `usize`-based identity semantics. Numeric identity does not itself encode scheduling priority. Uniqueness is a later `TaskSet` invariant rather than a scalar-construction rule.

## Relative deadlines

`RelativeDeadline` wraps a strictly positive `Duration`.

Use `RelativeDeadline::new(duration)` to validate positivity. A positive duration is retained exactly and `Duration::ZERO` returns `DeadlineError::Zero`.

The scalar enforces only its own positivity invariant. Cross-field task constraints such as execution time versus deadline versus period belong to the later task-parameter API and are not implemented here.

`RelativeDeadline` supports `Copy`, `Clone`, `Debug`, equality, and total ordering.

## Capacity accounting

`CapacityState` tracks an acquired-unit count against a validated `Limit`.

A new `CapacityState` starts at `used() == 0`. `try_acquire_one()` increments by one while capacity remains; otherwise it returns `CapacityError::Exhausted`. `release_one()` decrements by one when a unit is acquired; releasing from zero returns `CapacityError::ReleaseUnderflow`.

Failed transitions preserve the observable state.

`CapacityState` intentionally does not implement `Copy`, `Clone`, or `Default`; duplicating the value would duplicate accounting state.

## Work budgets

`Budget` is monotonic remaining-work accounting initialized from a validated `Limit`.

A new `Budget` starts with exactly the supplied `Limit` remaining. `try_consume_one()` decrements by one while work remains; otherwise it returns `BudgetError::Exhausted`.

A failed consumption preserves the remaining value.

`Budget` intentionally does not implement `Copy`, `Clone`, or `Default`.

## Error types

Current public errors are typed enums:

| Error | Variants |
|---|---|
| `LimitError` | `Zero` |
| `TimeArithmeticError` | `AdditionOverflow`, `MultiplicationOverflow`, `SubtractionUnderflow` |
| `CapacityError` | `Exhausted`, `ReleaseUnderflow` |
| `BudgetError` | `Exhausted` |
| `DeadlineError` | `Zero` |

These error enums support `Debug`, `Eq`, and `PartialEq`. They intentionally do not currently commit to `Copy`, `Clone`, `Display`, `core::error::Error`, blanket conversion traits, or serialization contracts.

Distinct material failure causes remain distinct. For example, capacity exhaustion and release underflow are separate variants, as are the three checked time-arithmetic failures.

## Foundation walkthrough

The following example exercises only the implemented foundation API:

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

This exact walkthrough is compiler- and Clippy-verified against the current implemented domain source snapshot.

## Failure-handling expectations

Callers should handle `Result` explicitly. The current foundation does not use sentinel values to represent arithmetic, capacity, budget, limit, or deadline failure.

For mutable accounting types, a failed transition leaves the observable state unchanged. For checked arithmetic, failure returns the exact applicable `TimeArithmeticError` instead of wrapping or saturating.

## Not available yet

The following are not part of the current public API:

- complete periodic task parameters;
- task sets;
- EDF, RM, or DM scheduler APIs;
- ready queues;
- schedulability-analysis APIs;
- deterministic simulation APIs;
- import/export contract codecs;
- browser interoperability;
- web UI;
- a published supported release/install contract.

Do not infer these APIs from scaffold paths, design documents, or roadmap text.

## Related documentation

- [Project overview](../README.md)
- [Architecture](ARCHITECTURE.md)
- [Design record](DESIGN.md)
- [Mathematical model](MATHEMATICAL_MODEL.md)
- [Memory and resource model](MEMORY_MODEL.md)
- [Error model](ERROR_MODEL.md)
- [Security policy](../SECURITY.md)
- [Contributing](../CONTRIBUTING.md)

