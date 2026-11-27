# SOV-TIME F01 ARCHITECTURE CANDIDATE

STATUS := WORKING_PHASE_F_FOUNDATION_RECORD
TARGET := docs/ARCHITECTURE.md
CURRENT_TARGET_BEFORE_IMAGE_SHA256 := e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
MUTATION_MODE := OPTIMISTIC_VERSION_CHECK
AUTHORIZED_PHASE := F01
BOUND_PROJECT_HEAD := 06379fbd0d664a76e8f06ff62525252be5dd583d
PROJECT := SOV-TIME
DATE := 2026-11-27

## 1. Purpose

Define the initial component ownership graph, permitted dependency direction, build-world boundaries, trust boundaries, and non-claims for SOV-TIME before implementation commitments are made.

This working foundation record describes intended architecture only. It does not claim that member manifests, source files, target builds, no-heap closure, browser behavior, framework selection, or scheduling algorithms are implemented or verified.

## 2. Current physical workspace

The root Cargo workspace declares six project members:

- crates/domain
- crates/application
- crates/contracts
- crates/browser_interop
- crates/web_ui
- apps/web

The architecture maps each physical member to one logical semantic owner. Path names are not themselves evidence of ownership; later source and manifest verification must reconcile actual syntax, effects, dependencies, features, and public APIs with these bindings.

## 3. Logical components

### SOV_TIME_DOMAIN

Path: crates/domain

Owner: PURE_DOMAIN_SEMANTICS

Responsibilities:
- validated scheduling-domain values and invariants;
- deterministic scheduling analysis;
- bounded deterministic simulation;
- typed domain outcomes and typed domain failures.

Boundary:
- no browser, DOM, JavaScript, filesystem, network, environment, wall-clock, randomness, telemetry, persistence, framework, or presentation responsibility;
- no dependency on any other SOV-TIME component in the initial architecture;
- production profile is a strict no_std/no_alloc/no-heap candidate only. The claim is not considered proven until the exact production dependency/unit graph, target build, resolved calls, native inputs, allocator state, and image evidence establish it.

### SOV_TIME_CONTRACTS

Path: crates/contracts

Owner: VERSIONED_BOUNDED_CONTRACT_TYPES

Responsibilities:
- bounded external/import/export representations;
- schema and version validity;
- transport-facing structural validation.

Boundary:
- no scheduling theorem or scheduling algorithm ownership;
- no browser handle or framework type ownership;
- no dependency on DOMAIN in the initial architecture;
- conversion from validated contract data into domain values is owned by APPLICATION, while DOMAIN constructors remain the final invariant boundary.

### SOV_TIME_APPLICATION

Path: crates/application

Owner: USE_CASE_ORCHESTRATION

Responsibilities:
- use-case orchestration;
- conversion from validated contract data to domain values;
- invocation of domain operations;
- mapping typed domain results to application-level results.

Allowed project dependencies:
- SOV_TIME_DOMAIN
- SOV_TIME_CONTRACTS

Boundary:
- no browser/DOM/JavaScript/framework/database vendor ownership;
- no replacement of domain invariants with presentation or transport validation.

### SOV_TIME_BROWSER_INTEROP

Path: crates/browser_interop

Owner: TYPED_BROWSER_INTEROP

Responsibilities:
- bounded browser file acquisition and export effects;
- browser storage only if separately admitted;
- browser/JavaScript interop necessary for approved client functionality;
- conversion between browser effects and validated bounded contract representations.

Allowed project dependency:
- SOV_TIME_CONTRACTS

Boundary:
- no direct scheduling/domain algorithm ownership;
- no reinterpretation of invalid contracts as valid;
- no direct dependency on DOMAIN or APPLICATION in the initial architecture.

### SOV_TIME_WEB_UI

Path: crates/web_ui

Owner: CLIENT_RENDERING_ROUTING_STATE

Responsibilities:
- rendering;
- navigation/routing;
- client interaction state;
- presentation mapping;
- user-visible distinction among typed outcomes, failures, unsupported results, and inconclusive results.

Allowed project dependencies:
- SOV_TIME_APPLICATION
- SOV_TIME_CONTRACTS
- SOV_TIME_BROWSER_INTEROP

Boundary:
- presentation/framework types stop here and do not leak inward;
- UI is not a security or domain source of truth;
- no server authority, persistence authority, or domain algorithm ownership.

### SOV_TIME_WEB_ENTRY

Path: apps/web

Owner: APPLICATION_ENTRY_COMPOSITION

Responsibilities:
- compose and launch the approved browser application;
- bind only the minimum qualified runtime/framework entry requirements.

Allowed project dependency:
- SOV_TIME_WEB_UI

Boundary:
- no direct business/domain algorithm ownership;
- direct dependencies on DOMAIN, APPLICATION, or BROWSER_INTEROP are denied by default unless a later architecture revision proves an explicit need and updates the component graph before implementation.

## 4. Dependency DAG

Initial permitted project dependency graph:

    SOV_TIME_DOMAIN  --------------------                                           > SOV_TIME_APPLICATION --    SOV_TIME_CONTRACTS ------------------/                        |
             |                                                     |
             +--> SOV_TIME_BROWSER_INTEROP ------------------------+--> SOV_TIME_WEB_UI --> SOV_TIME_WEB_ENTRY
             |                                                     |
             +-----------------------------------------------------+

Equivalent package edges:

- domain -> none
- contracts -> none
- application -> domain + contracts
- browser_interop -> contracts
- web_ui -> application + contracts + browser_interop
- web entry -> web_ui

The graph must remain acyclic. Any undeclared, reverse, or newly introduced edge requires architecture review before implementation commitment.

## 5. Prohibited dependency leakage

The following are prohibited by default:

- DOMAIN depending on contracts/application/browser/UI/entry/framework code;
- CONTRACTS depending on application/browser/UI/entry/framework code;
- APPLICATION depending on browser/UI/entry/framework vendor types;
- BROWSER_INTEROP depending directly on DOMAIN or APPLICATION;
- any lower layer depending on WEB_UI or WEB_ENTRY;
- Dioxus, wasm-bindgen, web-sys, DOM, or browser-only types entering DOMAIN, CONTRACTS, or APPLICATION without a separately reviewed boundary revision.

## 6. Build-world boundaries

### DOMAIN_FOUNDATION

Component: SOV_TIME_DOMAIN

Candidate profile: strict foundation profile.

Properties:
- deterministic explicit-input semantics;
- no runtime external effects;
- no browser/framework dependency;
- no_std/no_alloc/no-heap is a verification target, not a source-text assumption.

### HOST_TEST

Purpose: host-only evidence overlay.

Properties:
- may provide test-only capabilities when separately reviewed;
- must have zero production reachability;
- host success never substitutes for target-specific evidence.

### WEB_CLIENT_WASM

Components:
- CONTRACTS
- APPLICATION
- BROWSER_INTEROP
- WEB_UI
- WEB_ENTRY
- DOMAIN as an inward dependency of APPLICATION where the actual graph requires it.

Initial product candidate:
- browser client only;
- CSR/client execution candidate;
- no SSR/hydration requirement;
- no server executable;
- no server functions;
- no backend API;
- no database;
- no authentication/authorization subsystem because no privileged remote server resource is currently part of the candidate product.

These are architecture candidates, not permanent claims. If product requirements introduce server-side authority, persistence, SSR/hydration, or network APIs, F01 must be revised before those capabilities are implemented.

## 7. Framework boundary

The public project direction plans a Rust/WASM browser interface. Dioxus 0.7.10 has been researched as a technically plausible candidate, but no UI framework is admitted while the current Fullstack/WASM primary-profile selection remains ZERO_PRIMARY_PROFILE under RFW074.

Architecture does not make a framework version or framework choice normative.

Framework admission remains F07-owned and requires a lawful primary-profile recovery plus current evidence for requirement fit, exact version/pin, rendering model, interop model, maintenance posture, security/advisory state, supply-chain profile, performance/bundle budget, and migration cost.

Until F07 is lawfully recovered:
- Dioxus remains a candidate, not an admitted dependency;
- no framework dependency is permitted in DOMAIN, CONTRACTS, or APPLICATION;
- member manifests remain unmodified.

## 8. Trust-boundary data flow

Candidate flow:

UNTRUSTED_BROWSER_INPUT
  -> BROWSER_INTEROP_ACQUISITION
  -> VERSIONED_BOUNDED_CONTRACT_VALIDATION
  -> APPLICATION_USE_CASE_VALIDATION_AND_CONVERSION
  -> DOMAIN_VALUE_CONSTRUCTION
  -> PURE_DOMAIN_ANALYSIS_OR_SIMULATION
  -> APPLICATION_RESULT_MAPPING
  -> PRESENTATION_VIEW_MODEL_OR_VERSIONED_EXPORT_CONTRACT
  -> BROWSER_RENDER_OR_EXPLICIT_EXPORT_EFFECT

Rules:
- UI form values, imported files, and browser storage content are untrusted inputs.
- raw bytes/text are size-bounded before expensive parse or validation work;
- browser/JS values do not enter DOMAIN;
- contract validation does not substitute for domain invariant enforcement;
- domain output remains typed and presentation-neutral;
- no implicit cloud fallback, telemetry, remote error reporting, or network synchronization is part of the core path.

## 9. Failure direction

- Browser/file acquisition failure -> browser/application failure; zero domain mutation.
- Contract parse/schema/version failure -> rejected before domain conversion.
- Domain invariant/arithmetic/capacity failure -> preserve exact typed failure; UI may not coerce it to success.
- Unsupported or inconclusive analysis -> distinct from unschedulable.
- Export failure -> does not change an already computed analytical result.

## 10. Browser storage posture

Persistent browser storage is not required for the initial candidate product.

Any future localStorage, sessionStorage, IndexedDB, or equivalent persistence requires an explicit architecture revision covering:
- size and quota;
- schema/version migration;
- corruption behavior;
- privacy and lifetime;
- failure semantics;
- trust classification.

## 11. Architecture verification obligations

This F01 record establishes component/trust/dependency ownership only. Ongoing verification requires:

1. confirm or explicitly revise the component set and owners;
2. confirm or explicitly revise the client-only/no-backend product boundary;
3. prove the proposed project dependency graph is acyclic;
4. reconcile every component path to exactly one logical owner;
5. verify the trust-boundary and failure-direction model;
6. verify that no current requirement requires server authority/persistence;
7. retain framework selection as F07-owned until current evidence is evaluated;
8. keep no_std/no_alloc/no-heap language explicitly evidence-bound;
9. keep later framework/build claims blocked until their own gates pass.

## 12. Non-claims

This record does not claim:
- production readiness;
- implemented scheduling algorithms;
- verified schedulability guarantees;
- hardware WCET;
- no-heap proof;
- framework admission;
- target build pass;
- browser compatibility;
- security certification;
- server-side functionality.
