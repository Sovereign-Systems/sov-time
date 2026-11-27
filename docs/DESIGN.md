# SOV-TIME F01 DESIGN CANDIDATE

STATUS := WORKING_PHASE_F_FOUNDATION_RECORD
TARGET := docs/DESIGN.md
CURRENT_TARGET_BEFORE_IMAGE_SHA256 := e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
MUTATION_MODE := OPTIMISTIC_VERSION_CHECK
AUTHORIZED_PHASE := F01
BOUND_PROJECT_HEAD := 06379fbd0d664a76e8f06ff62525252be5dd583d
PROJECT := SOV-TIME
DATE := 2026-11-27

## 1. Design intent

SOV-TIME is designed as a deterministic scheduling-analysis system whose mathematical/domain semantics remain independent from browser APIs, UI framework types, persistence, and deployment concerns.

The design separates:
- bounded domain semantics;
- versioned external contracts;
- use-case orchestration;
- browser effects;
- UI presentation;
- application entry composition.

This working foundation record defines responsibility and interaction rules. Algorithm formulas, exact capacity bounds, memory budgets, source manifests, framework dependencies, and production tuples remain owned by later phases.

## 2. Core design principles

### Explicit inputs and deterministic outcomes

Domain behavior receives explicit validated inputs. Domain algorithms do not read wall-clock time, browser state, environment variables, network state, randomness, or implicit global configuration.

For the same valid inputs and declared algorithm/model configuration, domain behavior is designed to be repeatable.

### Typed failure over panic shortcuts

Normal failure is represented through typed results. First-party unwrap/expect and equivalent silent panic shortcuts are prohibited. Materially distinct failure causes remain distinct when they affect recovery, security, or diagnosis.

### Bounded work and bounded data

Every untrusted collection, imported payload, simulation horizon, iteration budget, and resource-sensitive operation must eventually bind to an explicit limit before implementation claims are accepted.

A bounded-work model is not a hardware WCET claim.

### Domain invariants are final at the domain boundary

Transport, application, and UI validation may reject invalid input early, but they do not replace domain constructors and domain invariants.

### Presentation is not domain truth

Text, localization, formatting, decimal percentages, and UI-specific state remain outside the domain layer. Domain results use exact typed values and outcomes.

## 3. Data ingress

Initial client-side ingress candidates:
- explicit UI form input;
- imported local file content;
- future separately admitted browser storage.

Ingress pipeline:
1. browser layer acquires bytes/text/value;
2. size/capacity guard applies before expensive work;
3. CONTRACTS validates representation/schema/version;
4. APPLICATION performs use-case validation and conversion;
5. DOMAIN constructors enforce domain invariants.

Browser object types and JavaScript values must not cross directly into DOMAIN.

## 4. Data egress

Domain outputs remain typed and presentation-neutral.

APPLICATION maps them to:
- application results;
- bounded export contracts where required;
- presentation-ready but non-framework-owned view data where appropriate.

WEB_UI owns human-readable formatting and interaction presentation.

BROWSER_INTEROP owns actual file/download/storage/browser effects.

An export-side effect failure does not retroactively alter the domain analytical result.

## 5. Scheduling model boundary

The current public project direction targets periodic-task scheduling and deterministic analysis/simulation on a single processor.

Exact supported policies, task models, arithmetic domains, theorem assumptions, scheduling proofs, and simulation semantics are not defined by this F01 document. They belong to the mathematical/domain phases and must be documented and tested before being advertised as implemented.

F01 therefore establishes only the architecture into which those semantics will be placed.

## 6. Contracts versus domain values

CONTRACTS owns external representations.

DOMAIN owns invariant-preserving scheduling values.

APPLICATION owns conversion.

Consequences:
- serialized/browser representations are not domain types merely because fields resemble domain fields;
- schema validity does not prove domain validity;
- a domain constructor may reject a structurally valid contract;
- transport versioning does not leak into core scheduling mathematics unless explicitly required by a domain contract.

## 7. Browser effects

BROWSER_INTEROP is the exclusive initial owner of browser-specific effects.

Examples of candidate effect classes:
- file selection/read;
- explicit export/download;
- optional future browser storage;
- low-level JS/DOM interop needed by approved client functionality.

It must not own:
- scheduling algorithms;
- scheduling policy;
- mathematical analysis;
- domain invariant decisions;
- UI rendering policy.

## 8. UI framework design

WEB_UI owns rendering, routing/navigation, and interaction state.

Dioxus 0.7.10 is a researched framework candidate, but framework admission and exact version/features remain deferred to F07 and blocked while the current requirement vector resolves to ZERO_PRIMARY_PROFILE under RFW074.

Therefore:
- F01 defines a framework boundary, not a framework dependency;
- no Dioxus-specific public type may enter DOMAIN, CONTRACTS, or APPLICATION;
- WEB_ENTRY stays a thin composition/launch boundary;
- rendering mode is initially a client-side/CSR candidate and must be explicit when F07/F08 bind the actual framework/build world.

## 9. No-backend initial candidate

The initial architecture candidate does not require:
- server executable;
- server functions;
- SSR/hydration;
- remote API;
- database;
- privileged server resource;
- authentication/authorization subsystem.

This is not a prohibition on future evolution. It is a guard against inventing server complexity before a product requirement exists.

Any future server capability requires architecture revision first, including trust boundary, authorization, persistence, deployment, failure, and data-flow changes.

## 10. Dependency discipline

Permitted initial dependencies:
- APPLICATION consumes DOMAIN and CONTRACTS;
- BROWSER_INTEROP consumes CONTRACTS;
- WEB_UI consumes APPLICATION, CONTRACTS, and BROWSER_INTEROP;
- WEB_ENTRY consumes WEB_UI.

DOMAIN and CONTRACTS are inward foundations and do not consume hosted layers.

The actual Cargo graph must later prove this direction. Documentation alone cannot establish dependency conformance.

## 11. Build and evidence separation

Distinct evidence classes remain separate:
- host compilation/tests;
- production target build;
- Cargo dependency/unit graph;
- selected Clippy results;
- browser tests;
- mathematical/model evidence;
- memory/stack evidence;
- benchmark evidence.

A pass in one class is not reclassified as proof in another.

## 12. no_std/no_alloc/no-heap design target

DOMAIN is intended to support a strict foundation profile.

However:
- no_std syntax alone does not prove no allocation;
- absence of allocator symbols alone does not prove no heap;
- hosted WEB_CLIENT_WASM packages may use std/allocation under their own bounded resource contracts;
- no hosted capability may leak into DOMAIN to weaken the foundation claim.

The final claim requires later graph/effect/target/image evidence.

## 13. Security and trust posture

Initial trust assumptions:
- all user/browser/imported data is untrusted;
- local origin does not make persisted data trusted;
- browser delivery and supply chain remain security boundaries;
- client-side validation is not a server authorization mechanism;
- because no server authority exists in the initial candidate, server authorization rules are not claimed satisfied by absence; they are currently outside the selected product topology.

Future network/server capabilities reopen the relevant Fullstack/WASM rules.

## 14. Design decisions and continuing gates

The Phase F target scope is now explicitly authorized. The following design decisions remain binding review points:

1. Does the owner retain the client-only/no-backend initial product topology?
2. Does each physical crate retain exactly one semantic owner?
3. Is the dependency DAG still the intended project direction?
4. Has any requirement emerged for persistence, SSR, hydration, server functions, API/backend, authentication, or database?
5. Does the framework boundary remain valid regardless of F07 framework choice?
6. Do architecture statements avoid converting candidate properties into unsupported implementation claims?
7. Does every later phase remain within its own dependency and source-admission gates?

## 15. F01 completion boundary

F01 completes when:
- target before-images are rechecked;
- the exact target contents are written and read back;
- internal claims are validated against current sources;
- F02 binds to the resulting exact F01 identities.

This working record is not a release, implementation, framework-admission, or engineering-baseline claim.
