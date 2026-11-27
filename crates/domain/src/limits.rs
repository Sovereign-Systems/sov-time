// sovereign-rust/v1 {"id":"domain.limits","purpose":"Represent finite validated limits that bound supported domain work."}

use crate::LimitError;

/// A strictly positive finite bound used to limit supported domain work.
///
/// `Limit` stores an exact `u32` value. It does not allocate storage, imply a
/// structural capacity, or authorize work by itself. Project-specific maximum
/// values are validated by the consuming operation.
///
/// # Errors
///
/// [`Limit::new`] returns [`LimitError::Zero`] when `raw` is zero.
///
/// # Panics
///
/// Construction and access do not panic.
pub struct Limit(u32);

impl Limit {
    /// Constructs a strictly positive finite limit.
    ///
    /// # Errors
    ///
    /// Returns [`LimitError::Zero`] when `raw` is zero.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn new(raw: u32) -> Result<Self, LimitError> {
        if raw == 0 {
            Err(LimitError::Zero)
        } else {
            Ok(Self(raw))
        }
    }

    /// Returns the exact stored bound.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn get(&self) -> u32 {
        self.0
    }
}
