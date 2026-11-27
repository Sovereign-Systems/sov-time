# SOV-TIME Mathematical Model — v0.1

STATUS := OWNER_BOUND_PHASE_F_WORKING_MODEL
PROJECT := SOV-TIME
MODEL_VERSION := 0.1
RESOURCE_PROFILE_BINDING := OWNER_BOUND_2026_11_27
RESOURCE_PROFILE_SHA256 := 38553424ef1a9df1e732cb49b0e1085a8eca01d8d3b135ad0b021dd267d537c0
OWNER_BINDING_PACKET_SHA256 := b5527b29e6ddd1774d11c4386ef14eadbb9c1431155e24f1f610385930749940
DEADLINE_MISS_CONTINUATION_POLICY := TERMINATE_ON_FIRST_MISS
TASK_ID_BINDING := OWNER_BOUND_2026_11_27_U32
TASK_ID_BACKING_TYPE := u32
DATE := 2026-11-27

## 1. Scope and non-claims

This document defines the mathematical semantics for the initial SOV-TIME domain. It is not an implementation, build result, hardware timing proof, WCET measurement, safety certification, or release claim.

The v0.1 model is deliberately narrow:
- one preemptive uniprocessor;
- independent periodic tasks;
- first release at time zero;
- abstract nonnegative integer ticks;
- constrained deadlines only: C <= D <= T;
- no release phase/offset;
- no jitter;
- no blocking or suspension;
- no precedence constraints;
- no parallel or multiprocessor execution;
- no modeled context-switch or scheduling overhead;
- no wall-clock or physical-unit conversion.

C is modeled execution demand. It is not a measured hardware WCET.

## 2. Time domain

Storage domain:
- Instant: u64 abstract ticks;
- Duration: u64 abstract ticks;
- RelativeDeadline: positive Duration.

All domain arithmetic is checked.

No arithmetic path may use wraparound, saturating arithmetic, floating-point approximation, panic, or a sentinel to hide representational failure.

For positive b, ceiling division is defined without the overflow-prone expression a+b-1:

    q = a / b
    r = a % b
    ceil_div(a,b) = q + [r != 0]

The final increment is checked.

Where a wider exact intermediate strictly increases safe headroom, checked u128 may be used as an intermediate. Conversion back into the declared u64 domain is checked. u128 does not remove the requirement for bounded work or typed overflow.

## 3. Periodic task model

Each task is:

    tau_i = (TaskId_i, C_i, T_i, D_i)

with:

    C_i > 0
    T_i > 0
    D_i > 0
    C_i <= D_i <= T_i

D_i = T_i identifies the implicit-deadline subset. No separate stored "implicit/constrained" profile flag is required; the relation is derived from values.

TaskId is semantic identity and the final deterministic tie-break key. Input slice position is not a scheduling priority.

Owner-bound v0.1 TaskId representation:
- backing/storage type: fixed-width `u32`;
- no reserved or sentinel TaskId value is defined; every `u32` bit pattern is a representable identity value;
- TaskId numeric value does not itself encode scheduling priority;
- TaskId public meaning must not depend on `usize` or architecture-dependent width;
- uniqueness remains a TaskSet invariant rather than a TaskId-construction invariant.

The empty task set is a valid domain value.

## 4. Runtime scheduling policy order

Deterministic keys:

EDF:
    (absolute_deadline, TaskId)

RM:
    (T, TaskId)

DM:
    (D, TaskId)

RM's optimal fixed-priority assignment claim is limited to the admitted implicit-deadline model.

DM's fixed-priority assignment claim is limited to the admitted constrained-deadline model.

There is no arbitrary user priority or stored fixed-priority rank in v0.1.

## 5. Exact utilization

For a task:

    U_i = C_i / T_i

For a task set:

    U = SUM_i U_i

The domain representation is an exact rational value with checked fixed-width integer arithmetic. No f32/f64, decimal percentage, JavaScript Number, or rounded display value participates in a schedulability decision.

Canonical representation is reduced by gcd so mathematically equal utilization values compare identically.

The empty set has utilization 0/1.

Representational overflow is a typed arithmetic failure, not a schedulability verdict.

## 6. Fixed-priority response-time analysis

Applicability:
- admitted v0.1 constrained task model;
- explicit fixed-priority policy RM or DM;
- higher-priority relation is defined by the same deterministic policy comparator used by runtime scheduling.

For task i:

    R_i^0 = C_i

    R_i^(n+1)
      = C_i
        + SUM_{j in hp(i)}
            ceil(R_i^n / T_j) * C_j

All multiplication, ceiling division, addition, and conversion operations are checked.

Decision semantics:
- if R_next > D_i: UNSCHEDULABLE with an exact response-time/deadline witness;
- if R_next = R_current and R_current <= D_i: task i is proved schedulable;
- otherwise continue within the explicit proof Budget;
- if Budget is exhausted before a proof or miss: INCONCLUSIVE;
- arithmetic/representation failure: typed error, no AnalysisOutcome is fabricated.

Whole-set SCHEDULABLE requires every task to be proved.

Arbitrary-deadline multi-job response-time analysis is outside v0.1.

## 7. Preemptive EDF for implicit deadlines

If every task has D = T, the exact feasibility path is:

    U <= 1

using exact rational utilization.

Candidate semantics:
- U <= 1: SCHEDULABLE under the admitted implicit-deadline preemptive uniprocessor model;
- U > 1: UNSCHEDULABLE with exact utilization witness.

DBF may be used as a bounded cross-check but is not required as the production fast path for this subset.

## 8. Preemptive EDF for constrained deadlines

For task i:

    dbf_i(t) = 0                         when t < D_i
    dbf_i(t) = (1 + floor((t-D_i)/T_i)) * C_i   otherwise

All operations are checked.

Synchronous busy-period recurrence:

    A^0 = SUM_i C_i

    A^(n+1)
      = SUM_i ceil(A^n / T_i) * C_i

Iteration continues until the least fixed point A is reached, subject to an explicit proof Budget.

Required demand candidate points are absolute-deadline points:

    t = D_j + k*T_j
    k >= 0
    t <= A

No LCM/hyperperiod is constructed and no unbounded candidate-point collection is allocated.

For every required point through A:

    SUM_i dbf_i(t) <= t

Candidate semantics:
- first point with total demand > t: UNSCHEDULABLE with exact witness {interval=t, demand, capacity=t};
- complete coverage through proved A with no violation: SCHEDULABLE;
- Budget exhausted before busy-period or point coverage completes: INCONCLUSIVE;
- checked arithmetic failure: typed error.

Duplicate candidate points may be evaluated more than once if the bounded-work accounting includes that cost; deduplication is not required for correctness.

## 9. Outcome semantics

AnalysisOutcome is semantically distinct from execution failure:

SCHEDULABLE
- emitted only after the complete applicable proof.

UNSCHEDULABLE
- emitted only with a concrete violated condition/witness.

UNSUPPORTED_MODEL
- valid data lies outside the assumptions of the selected exact theorem.

INCONCLUSIVE
- model is supported, but an explicit finite proof/resource bound was exhausted before proof completion.

Typed input, arithmetic, representation, or invariant failures are errors outside AnalysisOutcome.

A failed sufficient test must never be converted automatically into UNSCHEDULABLE.

Simulation is evidence about the simulation model; it is not substituted for theorem-based schedulability analysis.

## 10. Simulation mathematical boundary

Common simulation model already established:
- same validated periodic constrained-deadline task model;
- one preemptive processor;
- release origin at zero;
- horizon H covers [0,H);
- completion/deadline obligations at H are processed;
- releases at H are outside the executed horizon;
- completion exactly at a deadline is success.

DEADLINE_MISS_CONTINUATION_POLICY := TERMINATE_ON_FIRST_MISS

Selected v0.1 policy:
- terminate on the first observed deadline miss;
- due deadlines are evaluated before same-timestamp release admission;
- the first due incomplete job terminates with a deterministic witness;
- no same-timestamp successor release is admitted after that terminating miss;
- under D <= T, at most one live job per task exists in every continuing pre-miss state;
- continue-after-miss backlog/tardiness semantics are outside v0.1 and require a separate future model.

Common same-timestamp order before the policy-specific branch:
1. account prior-interval execution/completion;
2. evaluate due deadlines;
3. if any due job is incomplete, terminate with the deterministic first-miss witness;
4. otherwise admit releases in deterministic TaskId order;
5. dispatch/preempt using the selected policy.

## 11. Resource-bounded proof obligations

RESOURCE_PROFILE_BINDING_STATUS := OWNER_BOUND_2026_11_27

Bound v0.1 resource values:
- MAX_TASKS = 32
- MAX_READY_JOBS = 32
- MAX_ACTIVE_JOBS = 32
- MAX_TIMELINE_SEGMENTS = 16,384
- MAX_ANALYSIS_WORK_UNITS = 1,048,576
- MAX_SIMULATION_SEMANTIC_STEPS = 65,536
- hosted MAX_IMPORT_CONTRACT_TASKS = 32
- hosted MAX_IMPORT_BYTES = 65,536
- LIMIT_BACKING_TYPE = u32
- EXTERNAL_TIME_UNIT_SET = {TICK}

These values are explicitly Owner-bound for SOV-TIME v0.1.

Derived relations:
    MAX_ACTIVE_JOBS = MAX_READY_JOBS = MAX_TASKS = 32

    duplicate_comparisons_max
      = 32*31/2
      = 496

AnalysisWorkUnit meter:
- RTA: one unit per recurrence-update attempt plus one unit per higher-priority interference contribution.
- EDF busy period: one unit per task contribution plus one unit per iteration terminal/fixed-point comparison.
- EDF DBF: one unit per task demand contribution plus one unit per candidate-point aggregate comparison/advance.

A full all-task RTA recurrence pass is therefore bounded by:
    32 + 32*31/2 = 528 units

The project maximum analysis Budget 2^20 contains:
    floor(1,048,576 / 528) = 1,985
full-pass equivalents before resource exhaustion.

Budget exhaustion for a supported model remains INCONCLUSIVE, never UNSCHEDULABLE.

SimulationSemanticStep meter:
- one unit per committed semantic timestamp transition;
- all completion/deadline/release/dispatch work at one timestamp is part of that one transition;
- the charge is acquired before the transition is committed.

At 65,536 maximum steps and 32 admitted tasks, one full linear task scan per step is bounded by:
    65,536 * 32 = 2,097,152 task inspections.

Timeline output is separately bounded to 16,384 segments. Timeline capacity exhaustion may terminate before step Budget exhaustion. No one-segment-per-step equivalence is assumed.

MAX_IMPORT_BYTES is a raw acquisition bound, independent of codec selection. First-over-limit input is rejected before full parse.

Limit/count semantics use fixed-width u32, never usize by public meaning. Conversion to usize is checked at actual indexing/storage boundaries.

V1 external unit is TICK only. No physical time conversion is admitted in v0.1.

A source-operation bound is not a hardware WCET or browser-latency claim.

## 12. Independent reference vectors

These expected values were computed independently from any SOV-TIME Rust implementation and are test oracles.

### RTA-V001 — RM schedulable
- tau1=(C=1,T=4,D=4)
- tau2=(C=1,T=5,D=5)
- tau1 iterates [1,1]
- tau2 iterates [1,2,2]
- expected: SCHEDULABLE

### RTA-V002 — RM unschedulable
- tau1=(2,3,3)
- tau2=(2,5,5)
- tau1 [2,2]
- tau2 [2,4,6]
- 6 > 5
- expected: UNSCHEDULABLE with tau2 response/deadline witness

### RTA-V003 — DM schedulable
- tau1=(1,4,2)
- tau2=(2,5,5)
- tau1 [1,1]
- tau2 [2,3,3]
- expected: SCHEDULABLE

### RTA-V004 — DM miss with utilization below one
- tau1=(2,5,3)
- tau2=(4,8,6)
- utilization = 9/10
- tau1 [2,2]
- tau2 [4,6,8]
- expected: UNSCHEDULABLE with tau2 witness

### EDF-DBF-V001 — constrained unschedulable
- tau1=(2,5,2)
- tau2=(2,7,3)
- utilization = 24/35
- busy period [4,4], A=4
- candidate points: 2,3
- demand(2)=2
- demand(3)=4 > 3
- expected witness {interval=3,demand=4,capacity=3}

### EDF-DBF-V002 — constrained schedulable
- tau1=(1,4,2)
- tau2=(1,5,5)
- utilization = 9/20
- busy period [2,2], A=2
- candidate point: 2
- demand(2)=1
- expected: SCHEDULABLE

### EDF-U-V001 — implicit equality boundary
- tau1=(1,2,2)
- tau2=(1,2,2)
- utilization = 1
- expected: SCHEDULABLE

### EDF-U-V002 — implicit overload
- tau1=(1,2,2)
- tau2=(2,3,3)
- utilization = 7/6
- expected: UNSCHEDULABLE with exact utilization witness

Production tests must not call production analysis helpers to manufacture their own expected values.

## 13. Research inputs — non-authority

These references are retained as non-authority design/review inputs and do not become project authority by presence:

- M. Joseph and P. Pandya, "Finding Response Times in a Real-Time System", The Computer Journal 29(5), 1986, pp. 390-395. DOI: 10.1093/comjnl/29.5.390.
- N. Audsley, A. Burns, M. Richardson, K. Tindell, A. J. Wellings, "Applying new scheduling theory to static priority pre-emptive scheduling", Software Engineering Journal 8(5), 1993. DOI: 10.1049/sej.1993.0034.
- S. K. Baruah, A. K. Mok, L. E. Rosier, "Preemptively Scheduling Hard-Real-Time Sporadic Tasks on One Processor", 11th IEEE Real-Time Systems Symposium, 1990, pp. 182-190. DOI: 10.1109/REAL.1990.128746.

Before implementation, the exact theorem assumptions used by SOV-TIME must be reviewed against admitted evidence and the project model above.

## 14. Implementation follow-on obligations

F09 model serialization fixes the v0.1 semantic/resource contract; it does not claim implementation, build, browser performance, WCET, no-heap closure, or release readiness.

Before implementation claims:
- verify checked arithmetic representation limits against the implemented public surfaces;
- bind concrete constructor/API signatures without weakening this model;
- reconcile specialized source classification against actual Rust syntax/effects before algorithm source creation;
- preserve the owner-bound resource values and exact failure semantics unless a separately authorized versioned change succeeds;
- obtain target-specific build/memory/browser evidence at the later gates that own those claims.
