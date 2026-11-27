<!--
@version 0.1.0-draft.5
@description Project overview, architecture, development status, and verification boundaries.
-->

# SOV-TIME

Deterministic real-time scheduling and analysis engine in Rust, with a planned browser-based WebAssembly and Dioxus interface.

## Status

Early development. The workspace scaffold exists; the scheduling engine and web application are not yet implemented or verified. No release, benchmark result, real-time guarantee, or certification is claimed.

## Objectives

- Model bounded sets of periodic tasks with explicit execution times, periods, deadlines, and priorities.
- Analyze schedulability under explicitly documented scheduling models and assumptions.
- Simulate scheduling decisions and deadline outcomes deterministically within caller-supplied resource limits.
- Present analysis and simulation results through a browser application without making the domain engine depend on browser APIs.

The initial design targets a single-processor model. Supported scheduling policies, task models, arithmetic limits, and analysis assumptions will be documented and tested before they are advertised as implemented.

## Architecture

| Component | Responsibility |
| --- | --- |
| `crates/domain` | Pure scheduling, analysis, bounded simulation, and domain invariants |
| `crates/application` | Use-case orchestration and scenario evaluation |
| `crates/contracts` | Validated, versioned data contracts |
| `crates/browser_interop` | Browser-specific input/output effects |
| `crates/web_ui` | Dioxus presentation and interaction |
| `apps/web` | Web application entry point |

The foundation is designed for `no_std` and no allocation, including its runtime dependency graph. The hosted web layer has a separate resource and dependency boundary. The complete browser application is not claimed to be allocation-free.

## Engineering requirements

- Explicit arithmetic, capacity, termination, and failure semantics.
- No first-party `mod.rs`, `unwrap()`, `expect()`, or equivalent silent panic shortcuts.
- Typed errors and documented public API contracts.
- Explicit component ownership and dependency direction.
- Verification of the actual target and feature graph before resource or portability claims.
- Distinct evidence for host tests, target builds, model checking, benchmarks, and browser tests.

## Development

The repository is currently a scaffold. Do not interpret the presence of a source path as an implemented feature.

The root toolchain configuration selects Rust 1.99.1. The formatter configuration selects Rust 2024 parsing and formatting style. The Clippy configuration disallows exemptions for `unwrap` and `expect` in tests and const contexts and declares an empty allowed-type list. These files have been written; the effective compiler and lint invocation, complete toolchain provisioning, and target support remain unverified. The Clippy configuration does not enable the associated lints by itself.

Build and test commands will be published after the workspace manifests, toolchain, target matrix, and verification requirements have been established. Before a build, verify the effective destinations of compiler artifacts, Cargo caches and temporary files against the project-local output boundary.

See [CONTRIBUTING.md](CONTRIBUTING.md) for development rules and [SECURITY.md](SECURITY.md) for security reporting.

## Limitations

Scheduling analysis is valid only under its stated mathematical assumptions. A simulated schedule is not evidence of hardware timing, worst-case execution time, or suitability for a safety-critical deployment. Those claims require separate target-specific evidence.

## License

SOV-TIME is licensed under the MIT License. See [LICENSE](LICENSE) for the exact terms. This license does not imply a production-ready release or authorization to publish development changes.
