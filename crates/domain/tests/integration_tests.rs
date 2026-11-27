// sovereign-rust/v1 {"id":"domain.tests.integration","purpose":"Verify public domain foundation behavior against bound v0.1 contracts."}

use sov_time_domain::{
    Budget, BudgetError, CapacityError, CapacityState, DeadlineError, Duration, Instant, Limit,
    LimitError, RelativeDeadline, TaskId, TaskParameterError, TaskParameters, TaskSet,
    TaskSetError, TimeArithmeticError,
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

#[test]
fn relative_deadline_rejects_zero_duration() {
    let result = RelativeDeadline::new(Duration::ZERO);
    assert!(matches!(result, Err(DeadlineError::Zero)));
}

#[test]
fn relative_deadline_accepts_one_tick() {
    let result = RelativeDeadline::new(Duration::from_ticks(1));
    assert!(matches!(result, Ok(value) if value.duration().ticks() == 1));
}

#[test]
fn relative_deadline_accepts_maximum_tick_value() {
    let result = RelativeDeadline::new(Duration::from_ticks(u64::MAX));
    assert!(matches!(result, Ok(value) if value.duration().ticks() == u64::MAX));
}

#[test]
fn relative_deadline_order_matches_contained_duration() {
    let first = RelativeDeadline::new(Duration::from_ticks(1));
    let second = RelativeDeadline::new(Duration::from_ticks(2));

    assert!(first.is_ok());
    assert!(second.is_ok());
    if let (Ok(first), Ok(second)) = (first, second) {
        assert!(first < second);
    }
}

fn assert_public_error_traits<T: core::fmt::Debug + Eq>() {}

#[test]
fn public_error_types_support_diagnostic_value_semantics() {
    assert_public_error_traits::<LimitError>();
    assert_public_error_traits::<TimeArithmeticError>();
    assert_public_error_traits::<CapacityError>();
    assert_public_error_traits::<BudgetError>();
    assert_public_error_traits::<DeadlineError>();
}

#[test]
fn public_error_equality_preserves_variant_identity() {
    assert!(LimitError::Zero == LimitError::Zero);
    assert!(TimeArithmeticError::AdditionOverflow != TimeArithmeticError::MultiplicationOverflow);
    assert!(
        TimeArithmeticError::MultiplicationOverflow != TimeArithmeticError::SubtractionUnderflow
    );
    assert!(CapacityError::Exhausted != CapacityError::ReleaseUnderflow);
    assert!(BudgetError::Exhausted == BudgetError::Exhausted);
    assert!(DeadlineError::Zero == DeadlineError::Zero);
}

fn assert_task_id_traits<T: Copy + Clone + core::fmt::Debug + Eq>() {}

#[test]
fn task_id_round_trips_owner_bound_u32_domain() {
    let zero = TaskId::from_raw(0);
    let maximum = TaskId::from_raw(u32::MAX);

    let zero_raw: u32 = zero.get();
    let maximum_raw: u32 = maximum.get();

    assert!(zero_raw == 0);
    assert!(maximum_raw == u32::MAX);
}

#[test]
fn task_id_equality_is_exact_raw_identity() {
    let first = TaskId::from_raw(7);
    let same = TaskId::from_raw(7);
    let different = TaskId::from_raw(8);

    assert!(first == same);
    assert!(first != different);
}

#[test]
fn task_id_supports_minimal_value_traits() {
    assert_task_id_traits::<TaskId>();
}

fn make_valid_task(id: u32, execution: u64, deadline: u64, period: u64) -> Option<TaskParameters> {
    let deadline_value = match RelativeDeadline::new(Duration::from_ticks(deadline)) {
        Ok(value) => value,
        Err(_) => return None,
    };

    TaskParameters::new(
        TaskId::from_raw(id),
        Duration::from_ticks(execution),
        Duration::from_ticks(period),
        deadline_value,
    )
    .ok()
}

#[test]
fn task_parameters_preserve_exact_valid_fields() {
    let task = make_valid_task(7, 2, 4, 5);
    assert!(task.is_some());
    let Some(task) = task else {
        return;
    };

    assert!(task.id() == TaskId::from_raw(7));
    assert!(task.execution().ticks() == 2);
    assert!(task.deadline().duration().ticks() == 4);
    assert!(task.period().ticks() == 5);
}

#[test]
fn task_parameters_reject_zero_execution_and_zero_period_distinctly() {
    let deadline = RelativeDeadline::new(Duration::from_ticks(1));
    assert!(deadline.is_ok());

    if let Ok(deadline) = deadline {
        let zero_execution = TaskParameters::new(
            TaskId::from_raw(1),
            Duration::ZERO,
            Duration::from_ticks(1),
            deadline,
        );
        assert!(matches!(
            zero_execution,
            Err(TaskParameterError::ZeroExecution)
        ));
    }

    let deadline = RelativeDeadline::new(Duration::from_ticks(1));
    assert!(deadline.is_ok());

    if let Ok(deadline) = deadline {
        let zero_period = TaskParameters::new(
            TaskId::from_raw(1),
            Duration::from_ticks(1),
            Duration::ZERO,
            deadline,
        );
        assert!(matches!(zero_period, Err(TaskParameterError::ZeroPeriod)));
    }
}

#[test]
fn task_parameters_preserve_distinct_cross_field_failures() {
    let short_deadline = RelativeDeadline::new(Duration::from_ticks(1));
    assert!(short_deadline.is_ok());

    if let Ok(short_deadline) = short_deadline {
        let execution_exceeds_deadline = TaskParameters::new(
            TaskId::from_raw(1),
            Duration::from_ticks(2),
            Duration::from_ticks(2),
            short_deadline,
        );
        assert!(matches!(
            execution_exceeds_deadline,
            Err(TaskParameterError::ExecutionExceedsDeadline)
        ));
    }

    let long_deadline = RelativeDeadline::new(Duration::from_ticks(2));
    assert!(long_deadline.is_ok());

    if let Ok(long_deadline) = long_deadline {
        let deadline_exceeds_period = TaskParameters::new(
            TaskId::from_raw(1),
            Duration::from_ticks(1),
            Duration::from_ticks(1),
            long_deadline,
        );
        assert!(matches!(
            deadline_exceeds_period,
            Err(TaskParameterError::DeadlineExceedsPeriod)
        ));
    }
}

#[test]
fn task_parameters_accept_equality_boundaries() {
    let c_equals_d = make_valid_task(1, 2, 2, 3);
    assert!(c_equals_d.is_some());
    let Some(c_equals_d) = c_equals_d else {
        return;
    };
    let d_equals_t = make_valid_task(2, 2, 3, 3);
    assert!(d_equals_t.is_some());
    let Some(d_equals_t) = d_equals_t else {
        return;
    };

    assert!(c_equals_d.execution().ticks() == c_equals_d.deadline().duration().ticks());
    assert!(d_equals_t.deadline().duration().ticks() == d_equals_t.period().ticks());
}

#[test]
fn empty_task_set_is_valid() {
    let limit = Limit::new(1);
    assert!(limit.is_ok());

    if let Ok(limit) = limit {
        let tasks: [TaskParameters; 0] = [];
        let result = TaskSet::new(&tasks, limit);
        assert!(matches!(result, Ok(set) if set.tasks().is_empty()));
    }
}

#[test]
fn task_set_accepts_exact_caller_capacity_and_rejects_capacity_plus_one() {
    let first = make_valid_task(1, 1, 1, 2);
    assert!(first.is_some());
    let Some(first) = first else {
        return;
    };
    let second = make_valid_task(2, 1, 1, 2);
    assert!(second.is_some());
    let Some(second) = second else {
        return;
    };
    let exact_tasks = [first, second];

    let exact_limit = Limit::new(2);
    assert!(exact_limit.is_ok());
    if let Ok(exact_limit) = exact_limit {
        let result = TaskSet::new(&exact_tasks, exact_limit);
        assert!(matches!(result, Ok(set) if set.tasks().len() == 2));
    }

    let first = make_valid_task(1, 1, 1, 2);
    assert!(first.is_some());
    let Some(first) = first else {
        return;
    };
    let second = make_valid_task(2, 1, 1, 2);
    assert!(second.is_some());
    let Some(second) = second else {
        return;
    };
    let third = make_valid_task(3, 1, 1, 2);
    assert!(third.is_some());
    let Some(third) = third else {
        return;
    };
    let over_tasks = [first, second, third];

    let small_limit = Limit::new(2);
    assert!(small_limit.is_ok());
    if let Ok(small_limit) = small_limit {
        let result = TaskSet::new(&over_tasks, small_limit);
        assert!(matches!(result, Err(TaskSetError::CapacityExceeded)));
    }
}

#[test]
fn task_set_rejects_duplicate_identity_without_mutating_caller_slice() {
    let first = make_valid_task(9, 1, 1, 2);
    assert!(first.is_some());
    let Some(first) = first else {
        return;
    };
    let second = make_valid_task(9, 1, 1, 3);
    assert!(second.is_some());
    let Some(second) = second else {
        return;
    };
    let tasks = [first, second];

    let limit = Limit::new(2);
    assert!(limit.is_ok());

    if let Ok(limit) = limit {
        let result = TaskSet::new(&tasks, limit);
        assert!(matches!(
            result,
            Err(TaskSetError::DuplicateTaskId(id)) if id == TaskId::from_raw(9)
        ));
        let mut after = tasks.iter();
        assert!(matches!(
            after.next(),
            Some(task) if task.id() == TaskId::from_raw(9) && task.period().ticks() == 2
        ));
        assert!(matches!(
            after.next(),
            Some(task) if task.id() == TaskId::from_raw(9) && task.period().ticks() == 3
        ));
        assert!(after.next().is_none());
    }
}

#[test]
fn task_set_exposes_finite_read_only_input_order_without_priority_state() {
    let first = make_valid_task(20, 1, 1, 2);
    assert!(first.is_some());
    let Some(first) = first else {
        return;
    };
    let second = make_valid_task(10, 1, 1, 2);
    assert!(second.is_some());
    let Some(second) = second else {
        return;
    };
    let tasks = [first, second];

    let limit = Limit::new(2);
    assert!(limit.is_ok());

    if let Ok(limit) = limit {
        let result = TaskSet::new(&tasks, limit);
        assert!(result.is_ok());

        if let Ok(set) = result {
            let mut iter = set.tasks().iter();
            assert!(matches!(iter.next(), Some(task) if task.id() == TaskId::from_raw(20)));
            assert!(matches!(iter.next(), Some(task) if task.id() == TaskId::from_raw(10)));
            assert!(iter.next().is_none());
            assert!(set.limit().get() == 2);
        }
    }
}

#[test]
fn task_set_accepts_owner_maximum_of_thirty_two_unique_tasks() {
    let mut tasks = Vec::new();
    for id in 0_u32..32 {
        let task = make_valid_task(id, 1, 1, 2);
        assert!(task.is_some());
        let Some(task) = task else {
            return;
        };
        tasks.push(task);
    }

    let limit = Limit::new(32);
    assert!(limit.is_ok());

    if let Ok(limit) = limit {
        let result = TaskSet::new(tasks.as_slice(), limit);
        assert!(matches!(result, Ok(set) if set.tasks().len() == 32));
    }
}

#[test]
fn task_set_rejects_thirty_three_before_duplicate_semantics() {
    let mut tasks = Vec::new();
    for id in 0_u32..32 {
        let task = make_valid_task(id, 1, 1, 2);
        assert!(task.is_some());
        let Some(task) = task else {
            return;
        };
        tasks.push(task);
    }
    let duplicate_beyond_capacity = make_valid_task(0, 1, 1, 2);
    assert!(duplicate_beyond_capacity.is_some());
    let Some(duplicate_beyond_capacity) = duplicate_beyond_capacity else {
        return;
    };
    tasks.push(duplicate_beyond_capacity);

    let limit = Limit::new(33);
    assert!(limit.is_ok());

    if let Ok(limit) = limit {
        let result = TaskSet::new(tasks.as_slice(), limit);
        assert!(matches!(result, Err(TaskSetError::TaskCapacityExceeded)));
    }
}

#[test]
fn task_set_permutation_preserves_validity() {
    let first = make_valid_task(10, 1, 1, 2);
    assert!(first.is_some());
    let Some(first) = first else {
        return;
    };
    let second = make_valid_task(20, 1, 1, 3);
    assert!(second.is_some());
    let Some(second) = second else {
        return;
    };
    let first_order = [first, second];

    let first = make_valid_task(10, 1, 1, 2);
    assert!(first.is_some());
    let Some(first) = first else {
        return;
    };
    let second = make_valid_task(20, 1, 1, 3);
    assert!(second.is_some());
    let Some(second) = second else {
        return;
    };
    let second_order = [second, first];

    let first_limit = Limit::new(2);
    let second_limit = Limit::new(2);
    assert!(first_limit.is_ok());
    assert!(second_limit.is_ok());

    if let (Ok(first_limit), Ok(second_limit)) = (first_limit, second_limit) {
        assert!(TaskSet::new(&first_order, first_limit).is_ok());
        assert!(TaskSet::new(&second_order, second_limit).is_ok());
    }
}

fn assert_task_parameter_value_traits<T: core::fmt::Debug + Eq>() {}

#[test]
fn task_parameter_and_task_set_errors_support_diagnostic_equality() {
    assert_task_parameter_value_traits::<TaskParameters>();
    assert_public_error_traits::<TaskParameterError>();
    assert_public_error_traits::<TaskSetError>();
    assert!(TaskSetError::CapacityExceeded != TaskSetError::TaskCapacityExceeded);
    assert!(TaskSetError::TaskCapacityExceeded != TaskSetError::IndexConversionFailure);
}
