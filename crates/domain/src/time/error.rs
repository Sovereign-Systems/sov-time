// sovereign-rust/v1 {"id":"domain.time.error","purpose":"Represent typed checked time-arithmetic failures."}

/// Failure produced by checked abstract-tick arithmetic.
#[derive(Debug, Eq, PartialEq)]
pub enum TimeArithmeticError {
    /// An addition exceeded the representable `u64` tick domain.
    AdditionOverflow,
    /// A multiplication exceeded the representable `u64` tick domain.
    MultiplicationOverflow,
    /// A subtraction would produce a negative abstract tick value.
    SubtractionUnderflow,
}
