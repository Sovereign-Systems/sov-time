// sovereign-rust/v1 {"id":"domain.time.duration","purpose":"Represent an abstract scheduling duration in integral ticks."}

use super::TimeArithmeticError;

/// An abstract nonnegative duration expressed in integral ticks.
///
/// `Duration` carries no implicit physical unit. Every `u64` value, including
/// zero, is a valid generic duration; task-specific positivity is enforced by
/// the task-domain constructors that require it.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration(u64);

impl Duration {
    /// The zero-duration value.
    pub const ZERO: Self = Self(0);

    /// Constructs a duration from an exact abstract tick value.
    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    /// Returns the exact abstract tick value.
    pub const fn ticks(self) -> u64 {
        self.0
    }

    /// Adds another duration using checked arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TimeArithmeticError::AdditionOverflow`] if the exact result
    /// is greater than `u64::MAX`.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn checked_add(self, other: Self) -> Result<Self, TimeArithmeticError> {
        match self.0.checked_add(other.0) {
            Some(ticks) => Ok(Self(ticks)),
            None => Err(TimeArithmeticError::AdditionOverflow),
        }
    }

    /// Subtracts another duration using checked arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TimeArithmeticError::SubtractionUnderflow`] if `other` is
    /// greater than `self`.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn checked_sub(self, other: Self) -> Result<Self, TimeArithmeticError> {
        match self.0.checked_sub(other.0) {
            Some(ticks) => Ok(Self(ticks)),
            None => Err(TimeArithmeticError::SubtractionUnderflow),
        }
    }

    /// Multiplies this duration by an exact scalar using checked arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`TimeArithmeticError::MultiplicationOverflow`] if the exact
    /// result is greater than `u64::MAX`.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn checked_mul(self, factor: u64) -> Result<Self, TimeArithmeticError> {
        match self.0.checked_mul(factor) {
            Some(ticks) => Ok(Self(ticks)),
            None => Err(TimeArithmeticError::MultiplicationOverflow),
        }
    }
}
