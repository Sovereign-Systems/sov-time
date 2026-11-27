// sovereign-rust/v1 {"id":"domain.time","purpose":"Compose and explicitly reexport abstract scheduling-time primitives."}

mod duration;
mod error;
mod instant;

pub use duration::Duration;
pub use error::TimeArithmeticError;
pub use instant::Instant;
