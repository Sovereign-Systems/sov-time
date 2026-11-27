// sovereign-rust/v1 {"id":"domain.resource.budget","purpose":"Own monotonic remaining-work budget consumption semantics."}

use crate::Limit;

/// Failure produced by a work-budget transition.
#[derive(Debug, Eq, PartialEq)]
pub enum BudgetError {
    /// No work unit remains available for consumption.
    Exhausted,
}

/// Mutable monotonic accounting state for a finite work budget.
///
/// A new budget begins with exactly the supplied validated limit remaining.
/// Failed consumption preserves the prior remaining value. This type
/// intentionally does not implement `Copy`, `Clone`, or `Default`.
pub struct Budget {
    remaining: u32,
}

impl Budget {
    /// Creates a budget with exactly `limit` work units remaining.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn new(limit: Limit) -> Self {
        Self {
            remaining: limit.get(),
        }
    }

    /// Returns the exact number of work units remaining.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub const fn remaining(&self) -> u32 {
        self.remaining
    }

    /// Consumes exactly one work unit when one remains.
    ///
    /// # Errors
    ///
    /// Returns [`BudgetError::Exhausted`] when `remaining == 0`. On error,
    /// `remaining` is unchanged.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn try_consume_one(&mut self) -> Result<(), BudgetError> {
        if self.remaining == 0 {
            return Err(BudgetError::Exhausted);
        }

        match self.remaining.checked_sub(1) {
            Some(next) => {
                self.remaining = next;
                Ok(())
            }
            None => Err(BudgetError::Exhausted),
        }
    }
}
