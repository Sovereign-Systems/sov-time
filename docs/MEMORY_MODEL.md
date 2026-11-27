# SOV-TIME Memory Model — v0.1

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

## 1. Profile A memory objective

The SOV-TIME domain foundation targets a production closure with:

- no_std;
- no alloc dependency;
- no heap allocation;
- no global allocator dependency;
- no transitive std/alloc reachability in the qualified production graph;
- no runtime threads;
- recursion denied by default;
- bounded stack and bounded work for each supported exact target/build tuple.

These are verification targets, not claims established by this document.

Source appearance, no_std syntax, or absence of obvious Vec/Box use is insufficient proof.

## 2. Prohibited foundation ownership

The initial domain foundation does not own:

- Box
- Vec
- String
- Rc
- Arc
- heap-owning public result types
- hidden thread-local heap state
- runtime-created threads
- browser objects
- JavaScript values
- filesystem/network/environment effects
- ambient wall clock
- hidden unbounded caches

A future dependency or implementation that introduces such behavior reopens the memory/profile review before admission.

## 3. Borrowing and caller-owned storage

Preferred representation strategy:
- validated borrowed views over caller-owned data;
- caller-supplied bounded scratch where mutable working state is required;
- fixed-width scalar/value state for counters and identifiers;
- TaskId uses fixed-width `u32` backing and does not derive public identity meaning from `usize`.

TaskSet model:

    TaskSet<'a> = validated immutable view over &'a [TaskParameters] + explicit Limit

TaskSet does not copy the task array into an internal owned dynamic collection.

Construction validates cardinality before expensive duplicate scanning.

Caller input order is observational only and never becomes hidden scheduling priority.

## 4. Limit, capacity, and budget state

Limit:
- immutable positive finite bound value;
- does not itself allocate storage;
- does not prove a fixed array is sufficiently sized.

CapacityState:
- created from one Limit with used=0;
- constant-size mutable accounting state;
- not Copy/Clone/Default in the v0.1 design;
- failed acquire/release preserves prior state.

Budget:
- created from one Limit with remaining=limit;
- constant-size monotonic work-budget state;
- not Copy/Clone/Default;
- no ambient/global budget;
- distinct analyses/simulations own explicit Budget instances unless a consumer deliberately shares one.

Runtime limits and structural capacities must have an explicit relation when both exist.

## 5. Simulation scratch model

DEADLINE_MISS_CONTINUATION_POLICY := TERMINATE_ON_FIRST_MISS

Selected memory consequence:

    LIVE_JOBS_FOR_TASK_i <= 1
    LIVE_JOBS_TOTAL <= admitted_task_count

This follows from D_i <= T_i, due-deadline evaluation before same-timestamp release admission, and immediate termination on the first due incomplete job.

Scratch representation:

    caller scratch: one Option<Job> slot per admitted task

SimulationState borrows the scratch. The constructor verifies sufficient scratch capacity before mutating the admitted region.

A release into a nonempty same-task slot is an invariant contradiction.

Continue-after-miss backlog storage is outside v0.1 and is not admitted by this model.

## 6. Timeline storage

Timeline output uses separate caller-owned bounded scratch.

Safe representation:
- initialized Option<TimelineSegment> slots or another safe fixed representation;
- no unsafe is introduced merely to avoid Option overhead.

TimelineWriter owns logical length and append/merge state.

Timeline capacity is independent from:
- task capacity;
- active-job capacity;
- analysis proof Budget;
- simulation step Budget.

A full buffer may still accept an adjacent merge if no new slot is required.

If a new segment is required and no slot exists, capacity exhaustion is reported with writer state unchanged.

## 7. Capacity dimensions

RESOURCE_PROFILE_BINDING_STATUS := OWNER_BOUND_2026_11_27

Bound v0.1 resource profile:
- MAX_TASKS = 32
- MAX_READY_JOBS = 32
- MAX_ACTIVE_JOBS = 32
- MAX_TIMELINE_SEGMENTS = 16,384
- MAX_ANALYSIS_WORK_UNITS = 1,048,576
- MAX_SIMULATION_SEMANTIC_STEPS = 65,536
- hosted MAX_IMPORT_CONTRACT_TASKS = 32
- hosted MAX_IMPORT_BYTES = 65,536
- Limit backing = u32
- TaskId backing = u32

Selected first-miss model gives:
    LIVE_JOBS_TOTAL <= admitted_task_count <= 32

Therefore one caller-owned active-job slot per admitted task is structurally sufficient if implementation preserves the proved transition ordering.

Timeline storage remains a separate caller-owned capacity. Maximum logical timeline length is 16,384 segments; a smaller request is legal.

A full timeline buffer may still accept an adjacent merge if no new slot is consumed. A required new segment at capacity terminates without partial commit.

Analysis and simulation budgets do not allocate storage merely because their numeric limit is large; Budget/Limit accounting values are constant-size u32-backed state.

The 65,536-byte browser import bound is hosted-layer input buffering policy and does not imply equivalent foundation stack storage.

## 8. Stack and local-state discipline

Large fixed arrays are not automatically acceptable merely because they avoid the heap.

For each supported production tuple, evidence must identify:
- size and alignment of relevant values;
- local arrays/scratch references;
- simultaneously live stack objects;
- recursion absence or explicit bounded recursion proof;
- compiler/target lowering relevant to stack use;
- maximum supported structural capacities.

Host stack behavior does not prove WASM or another target's stack property.

## 9. Arithmetic representation and memory

Candidate stored time/task scalars use explicit fixed-width integers, not usize by convenience.

Current time storage candidate: u64 ticks.

Exact rational utilization and selected arithmetic may use checked u128 intermediates where useful.

The use of u128 is a correctness/representation decision only; target helper calls, code size, latency, and WCET require separate target evidence.

No LCM/hyperperiod array or unbounded proof-history vector is part of the candidate design.

## 10. Work bounds versus memory bounds

Memory-bounded does not automatically mean work-bounded.

Examples:
- TaskSet duplicate checking candidate is O(n^2), even though it allocates no heap.
- response-time analysis may perform repeated bounded scans; the private candidate recurrence bound D_i - C_i + 1 supports mathematical termination but does not replace product resource budgeting.
- DBF candidate-point enumeration avoids storing a point list but still requires an explicit Budget.
- simulation may use constant-size per-task scratch but still require an explicit step Budget.

F09/F10 must bind both storage and work budgets.

## 11. Hosted-layer separation

Hosted browser/application/UI layers may allocate under their separately qualified profile.

Hosted allocation does not weaken or prove the domain foundation profile.

Before conversion into domain values:
- imported/browser collections are size-bounded;
- schema/version is validated;
- capacities are checked;
- conversion is atomic.

Browser/UI types, alloc-owning containers, and framework state do not leak into the domain public ABI/API merely for convenience.

## 12. No-heap verification requirements

A positive no-heap claim requires evidence across the exact production tuple, including:
- exact dependency/unit/feature/source graph;
- production source effect analysis;
- generated/build-script closure;
- target compilation;
- symbol/section/relocation or equivalent linked-image evidence where applicable;
- allocator/global allocation reachability review;
- transitive std/alloc absence or exact permitted exception evidence;
- target-specific stack/resource checks;
- at/above-limit rejection tests.

No single check is sufficient.

## 13. Implementation and verification follow-on obligations

This memory model binds the v0.1 capacities and ownership strategy. It does not itself prove a compiled no-heap or stack-size claim.

Later implementation/verification shall:
- measure/review size and alignment for the final Rust types;
- bind caller scratch representations to the owner-bound capacities;
- prove the exact dependency graph after all member manifests and Cargo.lock exist;
- execute target-specific memory/stack/no-heap evidence for every tuple where such a claim is made;
- reject any future dependency or design change that would silently invalidate the bounded foundation model.
