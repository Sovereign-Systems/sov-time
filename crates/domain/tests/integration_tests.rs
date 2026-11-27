// sovereign-rust/v1 {"id":"domain.tests.integration","purpose":"Verify public domain foundation behavior against bound v0.1 contracts."}

use sov_time_domain::{
    Budget, BudgetError, CapacityError, CapacityState, Duration, Instant, Limit, LimitError,
    TimeArithmeticError,
};

#[test]
fn limit_accepts_minimum_positive_value() {
    let result = Limit::new(1);
    assert!(result.is_ok());

    if let Ok(limit) = result {
        assert!(limit.get() == 1);
    }
}

#[test]
fn limit_accepts_maximum_backing_value_without_truncation() {
    let result = Limit::new(u32::MAX);
    assert!(result.is_ok());

    if let Ok(limit) = result {
        assert!(limit.get() == u32::MAX);
    }
}

#[test]
fn limit_rejects_zero_with_exact_owned_error() {
    let result = Limit::new(0);
    assert!(matches!(result, Err(LimitError::Zero)));
}

#[test]
fn time_values_preserve_zero_and_maximum_ticks() {
    let zero_instant = Instant::from_ticks(0);
    let max_instant = Instant::from_ticks(u64::MAX);
    let zero_duration = Duration::ZERO;
    let max_duration = Duration::from_ticks(u64::MAX);

    assert!(zero_instant.ticks() == 0);
    assert!(max_instant.ticks() == u64::MAX);
    assert!(zero_duration.ticks() == 0);
    assert!(max_duration.ticks() == u64::MAX);
}

#[test]
fn checked_time_addition_reaches_maximum_without_wrapping() {
    let almost_max = 18_446_744_073_709_551_614_u64;
    let instant = Instant::from_ticks(almost_max);
    let duration = Duration::from_ticks(1);
    let duration_almost_max = Duration::from_ticks(almost_max);

    let instant_sum = instant.checked_add(duration);
    let duration_sum = duration_almost_max.checked_add(Duration::from_ticks(1));

    assert!(matches!(instant_sum, Ok(value) if value.ticks() == u64::MAX));
    assert!(matches!(duration_sum, Ok(value) if value.ticks() == u64::MAX));
}

#[test]
fn checked_time_addition_reports_exact_overflow() {
    let instant_result = Instant::from_ticks(u64::MAX).checked_add(Duration::from_ticks(1));
    let duration_result = Duration::from_ticks(u64::MAX).checked_add(Duration::from_ticks(1));

    assert!(matches!(
        instant_result,
        Err(TimeArithmeticError::AdditionOverflow)
    ));
    assert!(matches!(
        duration_result,
        Err(TimeArithmeticError::AdditionOverflow)
    ));
}

#[test]
fn checked_subtraction_reports_underflow_and_accepts_equal_values() {
    let duration_underflow = Duration::ZERO.checked_sub(Duration::from_ticks(1));
    let duration_equal = Duration::from_ticks(7).checked_sub(Duration::from_ticks(7));
    let instant_underflow = Instant::ZERO.checked_sub(Duration::from_ticks(1));

    assert!(matches!(
        duration_underflow,
        Err(TimeArithmeticError::SubtractionUnderflow)
    ));
    assert!(matches!(duration_equal, Ok(value) if value == Duration::ZERO));
    assert!(matches!(
        instant_underflow,
        Err(TimeArithmeticError::SubtractionUnderflow)
    ));
}

#[test]
fn generic_zero_duration_is_a_valid_value() {
    assert!(Duration::ZERO.ticks() == 0);
    assert!(Duration::from_ticks(0) == Duration::ZERO);
}

#[test]
fn checked_duration_multiplication_preserves_exact_results_and_overflow() {
    let zero = Duration::from_ticks(9).checked_mul(0);
    let identity = Duration::from_ticks(9).checked_mul(1);
    let maximum = Duration::from_ticks(1).checked_mul(u64::MAX);
    let overflow = Duration::from_ticks(2).checked_mul(u64::MAX);

    assert!(matches!(zero, Ok(value) if value == Duration::ZERO));
    assert!(matches!(identity, Ok(value) if value.ticks() == 9));
    assert!(matches!(maximum, Ok(value) if value.ticks() == u64::MAX));
    assert!(matches!(
        overflow,
        Err(TimeArithmeticError::MultiplicationOverflow)
    ));
}

#[test]
fn checked_duration_since_handles_equal_forward_and_reverse_instants() {
    let early = Instant::from_ticks(4);
    let late = Instant::from_ticks(9);

    let equal = early.checked_duration_since(early);
    let forward = late.checked_duration_since(early);
    let reverse = early.checked_duration_since(late);

    assert!(matches!(equal, Ok(value) if value == Duration::ZERO));
    assert!(matches!(forward, Ok(value) if value.ticks() == 5));
    assert!(matches!(
        reverse,
        Err(TimeArithmeticError::SubtractionUnderflow)
    ));
}

#[test]
fn time_value_order_matches_unsigned_tick_order() {
    assert!(Instant::ZERO < Instant::from_ticks(1));
    assert!(Instant::from_ticks(1) < Instant::from_ticks(u64::MAX));
    assert!(Duration::ZERO < Duration::from_ticks(1));
    assert!(Duration::from_ticks(1) < Duration::from_ticks(u64::MAX));
}

#[test]
fn checked_time_operations_are_repeatable_for_identical_inputs() {
    let first = Instant::from_ticks(10).checked_add(Duration::from_ticks(5));
    let second = Instant::from_ticks(10).checked_add(Duration::from_ticks(5));

    assert!(matches!(first, Ok(value) if value.ticks() == 15));
    assert!(matches!(second, Ok(value) if value.ticks() == 15));
}

#[test]
fn capacity_acquires_to_limit_then_preserves_state_on_exhaustion() {
    let result = Limit::new(2);
    assert!(result.is_ok());

    if let Ok(limit) = result {
        let mut capacity = CapacityState::new(limit);
        assert!(capacity.used() == 0);
        assert!(capacity.try_acquire_one().is_ok());
        assert!(capacity.used() == 1);
        assert!(capacity.try_acquire_one().is_ok());
        assert!(capacity.used() == 2);

        let exhausted = capacity.try_acquire_one();
        assert!(matches!(exhausted, Err(CapacityError::Exhausted)));
        assert!(capacity.used() == 2);
    }
}

#[test]
fn capacity_release_underflow_preserves_zero_state() {
    let result = Limit::new(1);
    assert!(result.is_ok());

    if let Ok(limit) = result {
        let mut capacity = CapacityState::new(limit);
        assert!(capacity.try_acquire_one().is_ok());
        assert!(capacity.release_one().is_ok());
        assert!(capacity.used() == 0);

        let underflow = capacity.release_one();
        assert!(matches!(underflow, Err(CapacityError::ReleaseUnderflow)));
        assert!(capacity.used() == 0);
    }
}

#[test]
fn capacity_interleaving_never_exceeds_bound() {
    let result = Limit::new(2);
    assert!(result.is_ok());

    if let Ok(limit) = result {
        let mut capacity = CapacityState::new(limit);
        assert!(capacity.try_acquire_one().is_ok());
        assert!(capacity.used() == 1);
        assert!(capacity.try_acquire_one().is_ok());
        assert!(capacity.used() == 2);
        assert!(capacity.release_one().is_ok());
        assert!(capacity.used() == 1);
        assert!(capacity.try_acquire_one().is_ok());
        assert!(capacity.used() == 2);
    }
}

#[test]
fn budget_consumes_exact_limit_then_preserves_zero_on_exhaustion() {
    let result = Limit::new(2);
    assert!(result.is_ok());

    if let Ok(limit) = result {
        let mut budget = Budget::new(limit);
        assert!(budget.remaining() == 2);
        assert!(budget.try_consume_one().is_ok());
        assert!(budget.remaining() == 1);
        assert!(budget.try_consume_one().is_ok());
        assert!(budget.remaining() == 0);

        let exhausted = budget.try_consume_one();
        assert!(matches!(exhausted, Err(BudgetError::Exhausted)));
        assert!(budget.remaining() == 0);
    }
}

#[test]
fn identical_budget_sequences_are_deterministic() {
    let first_limit = Limit::new(3);
    let second_limit = Limit::new(3);
    assert!(first_limit.is_ok());
    assert!(second_limit.is_ok());

    if let (Ok(first_limit), Ok(second_limit)) = (first_limit, second_limit) {
        let mut first = Budget::new(first_limit);
        let mut second = Budget::new(second_limit);

        assert!(first.try_consume_one().is_ok());
        assert!(second.try_consume_one().is_ok());
        assert!(first.try_consume_one().is_ok());
        assert!(second.try_consume_one().is_ok());
        assert!(first.remaining() == second.remaining());
        assert!(first.remaining() == 1);
    }
}
