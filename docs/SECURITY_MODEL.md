<!--
@version 0.1.0-draft.1
@description Public security model for the currently implemented SOV-TIME foundation and planned trust boundaries.
-->

# SOV-TIME Security Model

SOV-TIME is under active development and has no supported production release, security certification, or security audit.

This document separates controls that exist in the current domain foundation from trust boundaries that belong to later application, contract, browser, and UI work. Planned architecture is not presented as implemented security behavior.

## Security objectives

The current engineering model protects the following properties at the domain boundary:

- invalid values are rejected through explicit typed results where the current API defines an invariant;
- arithmetic overflow and underflow are reported instead of silently wrapping or saturating;
- capacity and work-budget exhaustion are explicit;
- failed mutable resource transitions preserve their prior observable state;
- materially distinct failures remain distinguishable;
- domain behavior depends on explicit inputs rather than ambient browser, filesystem, network, environment, wall-clock, randomness, telemetry, or persistence state;
- browser and presentation concerns do not become sources of domain truth.

These properties are correctness and containment controls. They are not a claim that the complete product is secure or production-ready.

## Current implemented security-relevant controls

### Positive finite limits

`Limit` rejects zero and stores an exact positive `u32` bound.

A `Limit` is accounting input. Its numeric range is not permission to allocate an equivalent amount of memory or perform an equivalent amount of work.

### Checked abstract time arithmetic

`Instant` and `Duration` use exact abstract `u64` ticks.

Current arithmetic operations use checked addition, subtraction, and multiplication. Representational failure produces a typed `TimeArithmeticError` instead of wraparound, saturation, a magic value, or an analytical verdict.

### Relative-deadline validation

`RelativeDeadline` rejects `Duration::ZERO`.

This scalar does not claim to validate the complete periodic-task relation. Cross-field constraints belong to the later task-parameter constructor.

### Capacity-state atomicity

`CapacityState` enforces bounded acquire/release accounting.

- acquiring at capacity returns `CapacityError::Exhausted`;
- releasing at zero returns `CapacityError::ReleaseUnderflow`;
- either failure leaves the used count unchanged.

The state intentionally does not implement `Copy`, `Clone`, or `Default`, avoiding accidental duplication of accounting state through those traits.

### Work-budget atomicity

`Budget` consumes one explicit work unit at a time.

- consumption at zero returns `BudgetError::Exhausted`;
- failed consumption leaves the remaining value unchanged;
- the state intentionally does not implement `Copy`, `Clone`, or `Default`.

A numeric budget is not a hardware WCET claim.

### Typed failure identity

The current public error enums preserve distinct material causes for:

- zero limit construction;
- addition overflow;
- multiplication overflow;
- subtraction underflow;
- capacity exhaustion;
- capacity release underflow;
- budget exhaustion;
- zero relative deadline.

Application or UI layers must not collapse materially different domain failures into a success value or unrelated analytical outcome.

## Trust boundaries

### Domain boundary

The domain layer owns invariant-preserving values and deterministic explicit-input semantics.

It does not own browser APIs, JavaScript values, file I/O, network I/O, environment variables, wall-clock sources, persistence, telemetry, framework state, or UI rendering.

A future higher layer may reject bad data earlier, but that does not replace domain constructors and domain invariants.

### External representation boundary

The planned contracts layer owns bounded external/import/export representations and schema/version validity.

That layer is not yet an implemented public codec surface. No serializer, parser, or transport format should be treated as security-reviewed merely because contract scaffolding exists.

### Browser-effect boundary

The planned browser-interoperability layer owns browser-specific acquisition and export effects.

Future form values, imported files, and browser-storage data are untrusted inputs. Raw external input must be bounded before expensive parsing or conversion, then validated before reaching domain computations.

The browser-interoperability implementation is not yet part of the current public feature set.

### UI boundary

The planned UI layer owns rendering and presentation state.

Presentation may explain a typed failure, but it must not redefine domain truth. In particular, unsupported, inconclusive, invalid, and unschedulable states must not be silently collapsed into one another once those higher-level outcomes are implemented.

## Threat classes considered by the architecture

The architecture is designed to address, where the corresponding feature exists:

- malformed or structurally invalid imported data;
- oversized external input;
- arithmetic overflow, underflow, and truncation;
- resource exhaustion;
- partial state mutation on failed operations;
- semantic error/outcome collapse across layers;
- hidden dependency or framework leakage into the domain foundation;
- unbounded analysis or simulation work;
- browser storage corruption or version mismatch;
- unsafe assumptions about host results proving WASM or hardware properties;
- unsupported claims derived from `no_std`, source appearance, or a single successful build.

Some of these are future obligations rather than implemented attack surfaces today.

## Current non-surfaces

The current public product does not yet expose:

- a backend API;
- server-side privileged resources;
- authentication or authorization;
- a database;
- SSR or server functions;
- network synchronization;
- telemetry or remote error reporting;
- a public import/export codec;
- browser storage behavior;
- a complete browser UI;
- implemented scheduler, schedulability-analysis, or simulation APIs.

Security requirements for those capabilities must be evaluated when and if the architecture admits them. Their absence today is not evidence that such controls are permanently unnecessary.

## Resource and denial-of-service posture

The owner-bound v0.1 model defines explicit limits for tasks, analysis work, simulation steps, timeline segments, and hosted import size. Those design bounds prevent later implementations from treating work and storage as implicitly unbounded.

Only the currently implemented `Limit`, `CapacityState`, and `Budget` primitives are public executable resource controls today.

A bounded count does not prove bounded CPU time, stack use, allocation behavior, or hardware WCET. Those properties require separate target-specific evidence.

## Dependency and supply-chain posture

The current `sov-time-domain` manifest has no external crate dependencies.

This is a narrow fact about the current domain foundation. It does not establish the dependency security of the future complete application, and it does not substitute for dependency-source, lockfile, advisory, license, or build-input review when external dependencies are admitted.

The UI framework remains unadmitted while the applicable framework/profile gate is unresolved.

## Failure safety

For every currently implemented fallible mutable resource transition, failure preserves the observable governed state.

Future stateful layers must retain the same preflight-before-commit discipline where partial effects would otherwise create inconsistent state.

An arithmetic or resource failure must not be converted into a successful scheduling or analytical verdict.

## Verification boundaries

Security-relevant changes should include the applicable negative and boundary tests for the exact revision and target.

Host verification does not prove browser, WASM, hardware timing, or release properties.

No single source scan, lint pass, `no_std` attribute, absence of an obvious allocation container, or successful compilation establishes a global no-heap or security guarantee.

## Vulnerability reporting

For reporting instructions, see the repository [Security policy](../SECURITY.md).

Do not publish credentials, private information, sensitive inputs, or exploit details in a public issue.

## Non-claims

This security model does not claim:

- production readiness;
- security certification;
- safety certification;
- complete attack-surface coverage;
- browser compatibility;
- dependency closure for the future application;
- a global no-heap proof;
- WCET;
- hardware real-time behavior;
- suitability for safety-critical or mission-critical use.

## Related documentation

- [Security policy](../SECURITY.md)
- [API guide](API_GUIDE.md)
- [Testing guide](TESTING.md)
- [Architecture](ARCHITECTURE.md)
- [Design record](DESIGN.md)
- [Memory and resource model](MEMORY_MODEL.md)
- [Error model](ERROR_MODEL.md)
- [Project overview](../README.md)
