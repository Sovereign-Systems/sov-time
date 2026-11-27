// sovereign-rust/v1 {"id":"domain.task.parameters","purpose":"Represent one immutable validated periodic constrained-deadline task tuple."}

use super::{RelativeDeadline, TaskId};
use crate::Duration;

/// Failure returned when constructing validated periodic task parameters.
#[derive(Debug, Eq, PartialEq)]
pub enum TaskParameterError {
    /// Execution demand must be strictly positive.
    ZeroExecution,
    /// Period must be strictly positive.
    ZeroPeriod,
    /// Execution demand must not exceed the relative deadline.
    ExecutionExceedsDeadline,
    /// The relative deadline must not exceed the period.
    DeadlineExceedsPeriod,
}

/// One immutable periodic task satisfying the SOV-TIME v0.1 task invariants.
///
/// The stored tuple is `(TaskId, C, T, D)` where `C > 0`, `T > 0`, and
/// `C <= D <= T`. Relative-deadline positivity is owned by [`RelativeDeadline`].
#[derive(Debug, Eq, PartialEq)]
pub struct TaskParameters {
    id: TaskId,
    execution: Duration,
    period: Duration,
    deadline: RelativeDeadline,
}

impl TaskParameters {
    /// Validates and constructs one periodic constrained-deadline task tuple.
    ///
    /// # Errors
    ///
    /// Returns [`TaskParameterError::ZeroExecution`] when `execution` is zero,
    /// [`TaskParameterError::ZeroPeriod`] when `period` is zero,
    /// [`TaskParameterError::ExecutionExceedsDeadline`] when `C > D`, and
    /// [`TaskParameterError::DeadlineExceedsPeriod`] when `D > T`.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn new(
        id: TaskId,
        execution: Duration,
        period: Duration,
        deadline: RelativeDeadline,
    ) -> Result<Self, TaskParameterError> {
        if execution.ticks() == 0 {
            return Err(TaskParameterError::ZeroExecution);
        }
        if period.ticks() == 0 {
            return Err(TaskParameterError::ZeroPeriod);
        }
        if execution.ticks() > deadline.duration().ticks() {
            return Err(TaskParameterError::ExecutionExceedsDeadline);
        }
        if deadline.duration().ticks() > period.ticks() {
            return Err(TaskParameterError::DeadlineExceedsPeriod);
        }

        Ok(Self {
            id,
            execution,
            period,
            deadline,
        })
    }

    /// Returns the stable task identity.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Returns the exact execution demand.
    pub fn execution(&self) -> Duration {
        self.execution
    }

    /// Returns the exact period.
    pub fn period(&self) -> Duration {
        self.period
    }

    /// Returns the exact validated relative deadline.
    pub fn deadline(&self) -> RelativeDeadline {
        self.deadline
    }
}
