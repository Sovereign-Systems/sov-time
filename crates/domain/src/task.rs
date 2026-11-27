// sovereign-rust/v1 {"id":"domain.task","purpose":"Compose and explicitly reexport validated task-domain scalar types."}

mod deadline;
mod identity;

pub use deadline::{DeadlineError, RelativeDeadline};
pub use identity::TaskId;
