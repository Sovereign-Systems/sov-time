// sovereign-rust/v1 {"id":"domain.resource","purpose":"Compose and explicitly reexport bounded resource-accounting state machines."}

mod budget;
mod capacity;

pub use budget::{Budget, BudgetError};
pub use capacity::{CapacityError, CapacityState};
