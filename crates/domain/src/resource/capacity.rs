// sovereign-rust/v1 {"id":"domain.resource.capacity","purpose":"Own bounded capacity acquire and release transition semantics."}

use crate::Limit;

/// Failure produced by a capacity-state transition.
pub enum CapacityError {
    /// No additional unit can be acquired without exceeding the configured limit.
    Exhausted,
    /// A release was requested while no unit was acquired.
    ReleaseUnderflow,
}

/// Mutable accounting state for a strictly positive finite capacity.
///
/// The invariant is `used <= limit`. Failed transitions leave the observable
/// state unchanged. This type intentionally does not implement `Copy`, `Clone`,
/// or `Default` because duplicating accounting state would duplicate capacity.
pub struct CapacityState {
    limit: u32,
    used: u32,
}

impl CapacityState {
    /// Creates empty capacity state for the supplied validated limit.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn new(limit: Limit) -> Self {
        Self {
            limit: limit.get(),
            used: 0,
        }
    }

    /// Returns the exact number of currently acquired units.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn used(&self) -> u32 {
        self.used
    }

    /// Acquires exactly one unit when capacity remains.
    ///
    /// # Errors
    ///
    /// Returns [`CapacityError::Exhausted`] when `used == limit`. On error,
    /// `used` is unchanged.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn try_acquire_one(&mut self) -> Result<(), CapacityError> {
        if self.used >= self.limit {
            return Err(CapacityError::Exhausted);
        }

        match self.used.checked_add(1) {
            Some(next) => {
                self.used = next;
                Ok(())
            }
            None => Err(CapacityError::Exhausted),
        }
    }

    /// Releases exactly one previously acquired unit.
    ///
    /// # Errors
    ///
    /// Returns [`CapacityError::ReleaseUnderflow`] when `used == 0`. On error,
    /// `used` is unchanged.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn release_one(&mut self) -> Result<(), CapacityError> {
        if self.used == 0 {
            return Err(CapacityError::ReleaseUnderflow);
        }

        match self.used.checked_sub(1) {
            Some(next) => {
                self.used = next;
                Ok(())
            }
            None => Err(CapacityError::ReleaseUnderflow),
        }
    }
}
