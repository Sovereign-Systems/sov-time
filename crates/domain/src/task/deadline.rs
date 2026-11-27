// sovereign-rust/v1 {"id":"domain.task.deadline","purpose":"Represent a strictly positive relative deadline over abstract duration ticks."}

use crate::Duration;

/// Failure returned when constructing a [`RelativeDeadline`].
#[derive(Debug, Eq, PartialEq)]
pub enum DeadlineError {
    /// A relative deadline must be strictly positive.
    Zero,
}

/// A strictly positive relative deadline expressed as an abstract [`Duration`].
///
/// This value carries no wall-clock, timezone, or physical-unit semantics.
/// Cross-field constraints such as execution-time <= deadline <= period are
/// owned by the later task-parameter constructor, not by this scalar wrapper.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RelativeDeadline(Duration);

impl RelativeDeadline {
    /// Constructs a strictly positive relative deadline.
    ///
    /// # Errors
    ///
    /// Returns [`DeadlineError::Zero`] when `duration` is [`Duration::ZERO`].
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn new(duration: Duration) -> Result<Self, DeadlineError> {
        if duration.ticks() == 0 {
            Err(DeadlineError::Zero)
        } else {
            Ok(Self(duration))
        }
    }

    /// Returns the exact contained abstract duration.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn duration(self) -> Duration {
        self.0
    }
}
