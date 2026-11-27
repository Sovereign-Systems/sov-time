// sovereign-rust/v1 {"id":"domain.root","purpose":"Compose the domain crate policy, owned modules, and deliberate minimal public facade."}

#![no_std]

//! Deterministic bounded scheduling-domain foundations for SOV-TIME.
//!
//! This crate owns pure explicit-input domain semantics. It does not own
//! browser, filesystem, network, environment, wall-clock, randomness,
//! telemetry, persistence, or UI-framework effects.
//!
//! The current public surface is limited to validated finite limits, abstract
//! checked time primitives, bounded resource-accounting state, stable task
//! identity, and a validated relative-deadline scalar. Scheduling, analysis,
//! and simulation APIs are not
//! yet exposed. No no-heap, timing,
//! WCET, or release claim is established by source syntax alone.

mod error;
mod limits;
mod resource;
mod task;
mod time;

pub use error::LimitError;
pub use limits::Limit;
pub use resource::{Budget, BudgetError, CapacityError, CapacityState};
pub use task::{DeadlineError, RelativeDeadline, TaskId};
pub use time::{Duration, Instant, TimeArithmeticError};
