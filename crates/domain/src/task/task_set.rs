// sovereign-rust/v1 {"id":"domain.task.set","purpose":"Represent a bounded borrowed immutable validated task-set view."}

use super::{TaskId, TaskParameters};
use crate::Limit;

const MAX_TASKS: u32 = 32;

/// Failure returned when constructing a bounded [`TaskSet`].
#[derive(Debug, Eq, PartialEq)]
pub enum TaskSetError {
    /// The caller slice exceeds the explicit caller-provided [`Limit`].
    CapacityExceeded,
    /// The observed task count exceeds the owner-bound v0.1 maximum of 32.
    TaskCapacityExceeded,
    /// A concrete storage count cannot be checked-converted to the fixed-width domain.
    IndexConversionFailure,
    /// Two tasks carry the same stable identity.
    DuplicateTaskId(TaskId),
}

/// A validated immutable borrowed view over periodic task parameters.
///
/// Construction enforces both the caller-provided [`Limit`] and the owner-bound
/// v0.1 maximum of 32 tasks before duplicate scanning. Effective admission is
/// therefore bounded by the smaller of the caller limit and 32. The caller
/// slice is borrowed directly; tasks are not copied into an owned dynamic
/// collection. After the 32-task admission cap, the deterministic duplicate
/// scan performs at most 496 prior-pair comparisons.
pub struct TaskSet<'a> {
    tasks: &'a [TaskParameters],
    limit: Limit,
}

impl<'a> TaskSet<'a> {
    /// Validates a bounded borrowed task set.
    ///
    /// Concrete slice cardinality is checked-converted to the fixed-width count
    /// domain before the owner cap, caller limit, and duplicate scan are
    /// evaluated. Empty slices are valid. Input position remains observational
    /// and never becomes scheduling priority.
    ///
    /// # Errors
    ///
    /// Returns [`TaskSetError::IndexConversionFailure`] when the concrete task
    /// count cannot be represented as `u32`,
    /// [`TaskSetError::TaskCapacityExceeded`] when the represented task count
    /// exceeds 32, [`TaskSetError::CapacityExceeded`] when the admitted count
    /// exceeds the explicit caller `limit`, and [`TaskSetError::DuplicateTaskId`]
    /// with the repeated identity when a duplicate is detected.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn new(tasks: &'a [TaskParameters], limit: Limit) -> Result<Self, TaskSetError> {
        let task_count = checked_task_count(tasks.len())?;

        if task_count > MAX_TASKS {
            return Err(TaskSetError::TaskCapacityExceeded);
        }

        if task_count > limit.get() {
            return Err(TaskSetError::CapacityExceeded);
        }

        for (offset, task) in tasks.iter().enumerate() {
            if tasks
                .iter()
                .take(offset)
                .any(|previous| previous.id() == task.id())
            {
                return Err(TaskSetError::DuplicateTaskId(task.id()));
            }
        }

        Ok(Self { tasks, limit })
    }

    /// Returns the exact caller-owned task slice as a read-only view.
    pub fn tasks(&self) -> &'a [TaskParameters] {
        self.tasks
    }

    /// Returns the exact caller-provided cardinality limit.
    ///
    /// This value is not the effective owner cap: admission is always limited
    /// to the smaller of this value and the v0.1 maximum of 32 tasks.
    pub fn limit(&self) -> &Limit {
        &self.limit
    }
}

fn checked_task_count(count: usize) -> Result<u32, TaskSetError> {
    match u32::try_from(count) {
        Ok(value) => Ok(value),
        Err(_) => Err(TaskSetError::IndexConversionFailure),
    }
}

#[cfg(all(test, target_pointer_width = "64"))]
mod tests {
    use super::{TaskSetError, checked_task_count};

    #[test]
    fn first_count_above_u32_domain_is_typed_conversion_failure() {
        let result = checked_task_count(4_294_967_296_usize);
        assert!(matches!(result, Err(TaskSetError::IndexConversionFailure)));
    }
}
