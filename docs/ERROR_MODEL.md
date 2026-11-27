# SOV-TIME Error Model — v0.1

STATUS := OWNER_BOUND_PHASE_F_WORKING_MODEL
PROJECT := SOV-TIME
MODEL_VERSION := 0.1
RESOURCE_PROFILE_BINDING := OWNER_BOUND_2026_11_27
RESOURCE_PROFILE_SHA256 := 38553424ef1a9df1e732cb49b0e1085a8eca01d8d3b135ad0b021dd267d537c0
OWNER_BINDING_PACKET_SHA256 := b5527b29e6ddd1774d11c4386ef14eadbb9c1431155e24f1f610385930749940
DEADLINE_MISS_CONTINUATION_POLICY := TERMINATE_ON_FIRST_MISS
DATE := 2026-11-27

## 1. Error-model objective

SOV-TIME preserves materially distinct failure causes through typed ownership boundaries.

A failure is not converted into:
- panic;
- unwrap/expect;
- boolean false;
- default value;
- empty string;
- magic numeric sentinel;
- saturation/wraparound;
- UNSCHEDULABLE;
- INCONCLUSIVE;
unless that semantic state is exactly what the owning contract defines.

There is no speculative catch-all DomainError in the initial design.

## 2. Error versus analysis outcome

Execution/validation failures and analytical outcomes are separate channels.

Analysis outcomes:
- Schedulable
- Unschedulable(Witness)
- UnsupportedModel(Reason)
- Inconclusive(Reason)

Outer typed errors cover invalid input, arithmetic/representation failure, or operation-specific invariant failure.

Rules:
- invalid input for a declared model is rejected before theorem execution;
- valid data outside theorem assumptions -> UnsupportedModel;
- a proved violated condition -> Unschedulable;
- bounded proof exhaustion with otherwise valid state -> Inconclusive;
- arithmetic failure -> typed error and no analytical verdict.

## 3. Ownership law

Each semantic module owns the narrow failure type required by its actual responsibility.

A crate-level aggregate error is permitted only when a real consumer API requires one channel. It must preserve the owned typed cause or an exact cause code with equivalent recovery semantics.

Stringification, boolean collapse, default fallback, and dropping material causes are prohibited.

Infallible From conversion is permitted only when no material distinction can be erased. Otherwise mapping is explicit and cause-preserving.

INTERNAL_INVARIANT_FAILURE, if ever introduced, is not an ordinary caller-input variant and cannot be a generic escape hatch.

## 4. Candidate owned failure domains

These names are design candidates and do not yet define final public enums.

### Limit construction
Owner: limits.rs / narrow foundational error support.

Candidate causes:
- zero when zero is invalid for the selected Limit contract;
- out-of-range when the selected raw/backing domain requires it.

The backing type is owner-bound to u32; final public enum naming remains an implementation/API decision that shall preserve these causes.

### Time arithmetic
Owner: time/error.rs.

Initial candidate causes:
- AdditionOverflow
- MultiplicationOverflow
- SubtractionUnderflow

No speculative division/conversion variants are added without an actual operation.

### Relative deadline
Owner: task/deadline.rs.

Candidate:
- Zero

### Task parameters
Owner: task/parameters.rs.

Candidate:
- ZeroExecution
- ZeroPeriod
- ExecutionExceedsDeadline
- DeadlineExceedsPeriod

Deadline construction failure remains owned by RelativeDeadline; it is not duplicated into TaskParameterError.

### TaskSet
Owner: task/task_set.rs.

Candidate:
- CapacityExceeded
- DuplicateTaskId(TaskId)

Caller storage remains unchanged on failure.

### Capacity state
Owner: resource/capacity.rs.

Candidate:
- Exhausted
- ReleaseUnderflow

Every failed transition preserves the prior used count.

### Budget
Owner: resource/budget.rs.

Candidate:
- Exhausted

Failed consumption preserves remaining state.

### Exact utilization
Owner: analysis/utilization.rs.

Candidate:
- ArithmeticOverflow

No partial ratio is returned.

### Fixed-priority response-time analysis
Owner: analysis/response_time.rs after final actual-syntax/effect classification confirms the planned FULLSTACK_DOMAIN_MODULE role.

Candidate typed execution failures:
- checked arithmetic overflow;
- checked representation/conversion failure.

Proof-budget exhaustion is not an execution error; it is Inconclusive with bounded progress payload.

### EDF demand-bound analysis
Owner: analysis/demand_bound.rs after final actual-syntax/effect classification confirms the planned FULLSTACK_DOMAIN_MODULE role.

Candidate typed execution failures:
- busy-period arithmetic overflow;
- candidate-point arithmetic overflow;
- demand arithmetic overflow;
- checked representation/conversion failure.

Busy-period or point-evaluation Budget exhaustion is Inconclusive, not an error.

### Simulation
Owners: job/state/timeline/engine modules according to transition ownership.

Candidate typed failures include:
- checked release/deadline/completion arithmetic failure;
- invalid execution amount;
- scratch/capacity rejection before commit;
- timeline invariant violation.

StepBudgetExhausted and TimelineCapacityExhausted are explicit semantic/resource termination states, not generic arithmetic errors.

DeadlineMiss is an explicit terminating semantic state with a deterministic witness under the owner-selected v0.1 first-miss policy.

A continue-after-miss model is outside v0.1 and would require a distinct error/event/report contract.

## 5. Atomicity rule

For every fallible mutation:

    failure => externally observable governed state equals pre-call state

Examples:
- CapacityState acquire at full capacity leaves used unchanged.
- CapacityState release at zero leaves used unchanged.
- Budget consume at zero leaves remaining unchanged.
- over-capacity TaskSet construction returns no view and mutates no caller storage.
- timeline append requiring a missing slot leaves writer unchanged.
- simulation next-step preflight fails before state/timeline commit.

The simulation engine design uses preflight-then-commit:
1. compute all fallible next-step information;
2. verify output capacity and Budget;
3. only then commit state/timeline transition.

No partial-success verdict is emitted after arithmetic or resource failure.

## 6. Panic policy

Recoverable expected failure must not be implemented with:
- panic!
- unreachable!
- unimplemented!
- todo!
- unwrap()
- expect()
- indexing that can panic on untrusted/runtime-controlled input
- equivalent hidden panic paths.

Test assertion framework panics do not by themselves violate production error semantics, but test support may not use panic shortcuts to model a production recovery path.

## 7. Public API policy

Private-by-default.

Only errors needed by explicit downstream consumer contracts are reexported.

A broad public enum is not introduced merely because several internal modules can fail.

Exact derives, non_exhaustive use, source/error trait integration, and conversion traits remain unresolved until the consumer surface is bound.

Error messages intended for UI display belong to hosted/application/UI mapping. Domain error identity must not depend on allocated diagnostic strings.

## 8. Cross-layer preservation

Application/contracts/UI may add bounded presentation context, but must preserve:
- error vs outcome distinction;
- exact AnalysisOutcome variant;
- witness/reason identity;
- material error cause;
- arithmetic vs model vs capacity vs resource distinction.

The UI may not turn Inconclusive or UnsupportedModel into Unschedulable.

Transport/export logic may not silently erase a newer semantic field to fit an older schema.

## 9. Negative-test obligations

Every owned error requires tests for:
- exact triggering boundary;
- exact owned error class/cause;
- forbidden side effects;
- unchanged state after rejected mutation where state exists.

Cross-layer mapping tests prove no semantic collapse.

Arithmetic overflow tests require typed arithmetic failure and zero partial verdict.

Model-applicability tests distinguish:
- invalid input;
- UnsupportedModel;
- Unschedulable;
- Inconclusive.

## 10. Implementation/API follow-on obligations

F09 error-model serialization is complete for the owner-bound v0.1 semantic/resource contract.

Later implementation/API work must:
- choose final public enum/type names without collapsing the owned causes defined here;
- choose exact representation payload structs for bounded progress and witnesses while preserving their semantics;
- reconcile final actual-content source classification for specialized algorithm units;
- preserve the owner-bound u32 Limit semantics and project capacity/Budget bounds;
- add any source/error trait integration only when a real consumer surface requires it.

These are implementation/API obligations, not unresolved F09 resource/model decisions.


## 11. Owner-bound resource-profile failures

RESOURCE_PROFILE_BINDING_STATUS := OWNER_BOUND_2026_11_27

The bound v0.1 resource profile defines the following explicit typed contract/domain causes:

TaskCapacityExceeded
- observed task count > 32;
- rejection occurs before duplicate scan/domain state construction commit.

ImportByteLimitExceeded
- raw browser acquisition bytes > 65,536;
- rejection occurs before full decode/parse.

AnalysisBudgetOutOfRange
- requested proof Budget is 0 or > 1,048,576.

SimulationStepBudgetOutOfRange
- requested semantic-step Budget is 0 or > 65,536.

TimelineCapacityOutOfRange
- requested timeline capacity is 0 or > 16,384.

AnalysisBudgetExhausted
- the next deterministic proof-work charge cannot be acquired;
- supported analysis returns Inconclusive, not an unschedulable verdict.

SimulationStepBudgetExhausted
- the next semantic timestamp transition cannot acquire one step charge;
- no partial transition is committed.

TimelineCapacityExhausted
- a nonmerge segment is required but no timeline slot remains;
- no partial segment/state transition is committed.

InvalidExternalTimeUnit
- any external unit other than TICK is rejected in v0.1.

IndexConversionFailure
- a fixed-width count cannot be checked-converted for the concrete storage/index boundary;
- no truncation or architecture-dependent reinterpretation is permitted.

The u32 backing representation is semantic capacity state, not permission to allocate u32::MAX elements.

## 12. Exact boundary table

- task_count 32: allowed subject to remaining invariants.
- task_count 33: TaskCapacityExceeded.
- import bytes 65,536: allowed to continue validation.
- import bytes 65,537: ImportByteLimitExceeded before full parse.
- analysis Budget 1: valid.
- analysis Budget 1,048,576: valid.
- analysis Budget 0 or 1,048,577: AnalysisBudgetOutOfRange.
- simulation steps 1: valid.
- simulation steps 65,536: valid.
- simulation steps 0 or 65,537: SimulationStepBudgetOutOfRange.
- timeline capacity 1: valid.
- timeline capacity 16,384: valid.
- timeline capacity 0 or 16,385: TimelineCapacityOutOfRange.

These boundaries are explicitly Owner-bound for SOV-TIME v0.1.
