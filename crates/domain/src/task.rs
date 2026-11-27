// sovereign-rust/v1 {"id":"domain.task","purpose":"Compose and explicitly reexport validated task-domain scalar and bounded task-set types."}

mod deadline;
mod identity;
mod parameters;
mod task_set;

pub use deadline::{DeadlineError, RelativeDeadline};
pub use identity::TaskId;
pub use parameters::{TaskParameterError, TaskParameters};
pub use task_set::{TaskSet, TaskSetError};
