// sovereign-rust/v1 {"id":"domain.task.identity","purpose":"Represent stable task identity with owner-bound fixed-width u32 storage."}

/// Stable semantic identity for one periodic task.
///
/// `TaskId` stores an exact owner-bound `u32` value. Every `u32` bit pattern
/// is representable; no value is reserved as a sentinel. Numeric identity does
/// not itself encode scheduling priority, and uniqueness is enforced by the
/// task-set boundary rather than by this scalar value.
///
/// # Panics
///
/// Construction and access do not panic.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TaskId(u32);

impl TaskId {
    /// Constructs a task identity from its exact raw `u32` representation.
    ///
    /// Every `u32` value is valid, including zero and [`u32::MAX`].
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the exact stored identity value.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn get(self) -> u32 {
        self.0
    }
}
