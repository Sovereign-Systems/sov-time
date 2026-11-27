// sovereign-rust/v1 {"id":"domain.error","purpose":"Represent typed domain failures without erasing material causes."}

/// Error returned when constructing a finite [`crate::Limit`].
///
/// The error is deliberately narrow: zero is the only invalid value for the
/// reusable positive finite limit primitive. Project-specific maximum values
/// are separate consuming-project bindings.
pub enum LimitError {
    /// The requested limit was zero, but a limit must be strictly positive.
    Zero,
}
