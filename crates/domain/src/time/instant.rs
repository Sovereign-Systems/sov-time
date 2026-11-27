// sovereign-rust/v1 {"id":"domain.time.instant","purpose":"Represent an abstract scheduling instant in integral ticks."}

use super::{Duration, TimeArithmeticError};

/// An abstract nonnegative scheduling instant expressed in integral ticks.
///
/// `Instant` is not a wall clock, monotonic clock source, timezone-aware
/// timestamp, or physical-time quantity. Every `u64` value is representable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Instant(u64);

impl Instant {
    /// The abstract instant at tick zero.
    pub const ZERO: Self = Self(0);

    /// Constructs an instant from an exact abstract tick value.
    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    /// Returns the exact abstract tick value.
    pub const fn ticks(self) -> u64 {
        self.0
    }

    /// Adds an abstract duration using checked arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TimeArithmeticError::AdditionOverflow`] if the exact result
    /// is greater than `u64::MAX`.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn checked_add(self, duration: Duration) -> Result<Self, TimeArithmeticError> {
        match self.0.checked_add(duration.ticks()) {
            Some(ticks) => Ok(Self(ticks)),
            None => Err(TimeArithmeticError::AdditionOverflow),
        }
    }

    /// Subtracts an abstract duration using checked arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TimeArithmeticError::SubtractionUnderflow`] if the duration
    /// is greater than this instant.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn checked_sub(self, duration: Duration) -> Result<Self, TimeArithmeticError> {
        match self.0.checked_sub(duration.ticks()) {
            Some(ticks) => Ok(Self(ticks)),
            None => Err(TimeArithmeticError::SubtractionUnderflow),
        }
    }

    /// Computes the checked duration elapsed since an earlier abstract instant.
    ///
    /// # Errors
    ///
    /// Returns [`TimeArithmeticError::SubtractionUnderflow`] when `earlier`
    /// is later than `self`.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn checked_duration_since(
        self,
        earlier: Self,
    ) -> Result<Duration, TimeArithmeticError> {
        match self.0.checked_sub(earlier.0) {
            Some(ticks) => Ok(Duration::from_ticks(ticks)),
            None => Err(TimeArithmeticError::SubtractionUnderflow),
        }
    }
}
