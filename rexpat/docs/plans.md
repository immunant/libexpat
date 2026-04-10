# rexpat refactoring plan

## Objectives

- Refactor `rexpat/*` toward idiomatic Rust while preserving the exported C ABI and the current test behavior.
- Reduce raw-pointer and `unsafe` usage to the FFI boundary and the smallest internal regions that are still required.
- Keep each change self-contained, reviewable, and validated with formatting plus the existing C-interface test suite.

## Global constraints

- Do not modify any source under `expat/`.
- Preserve `extern "C"` function signatures, `#[no_mangle]` symbol names, and `#[repr(C)]` layouts/field order for C-facing types.
- Avoid unwinding across FFI boundaries.
- Keep the test harness exercising the library through the C interface.
- Use the pinned toolchain from `rexpat/rust-toolchain.toml` (`nightly-2023-04-15`) for all verification.

## Baseline verification

Run these before the first implementation milestone and after any invasive refactor to confirm the workspace still matches expectations:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 1: Establish a safe internal refactoring perimeter

Scope:
Create a small set of internal helper abstractions and documentation that make later refactors mechanically safer without changing the exported API.

Key files/modules:
- `rexpat/lib.rs`
- `rexpat/src/lib/xmlparse.rs`
- `rexpat/src/lib/xmltok.rs`
- `rexpat/src/lib/xmlrole.rs`

Planned work:
- Inventory the major FFI entry points and the main internal unsafe patterns used by the translated code.
- Introduce private helper utilities and type aliases for repeated pointer arithmetic, sentinel range checks, and byte access patterns where they can be shared without changing ABI-visible types.
- Add targeted safety comments for the unsafe regions that remain necessary after these helper extractions.
- Remove obviously mechanical C2Rust artifacts at module boundaries where that can be done without semantic risk.

Acceptance criteria:
- No public ABI-visible signatures or C-facing struct layouts change.
- Internal helper layer exists and is documented well enough to support later milestones.
- The code builds and tests unchanged in behavior.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 2: Refactor `xmlrole` into idiomatic state-machine Rust

Scope:
Use the smallest module first to convert C-style control flow and data access into cleaner Rust while preserving parser behavior.

Key files/modules:
- `rexpat/src/lib/xmlrole.rs`

Planned work:
- Replace C2Rust-style integer-state branching and repeated match fallthrough patterns with clearer Rust control flow where behavior is unchanged.
- Consolidate duplicated keyword and token handling helpers.
- Narrow unsafe blocks to individual pointer dereferences or FFI calls instead of whole functions where practical.
- Keep `PROLOG_STATE` and related exported constants/types ABI-stable.

Acceptance criteria:
- `xmlrole.rs` is materially more readable and uses smaller, justified unsafe regions.
- State transitions remain behaviorally identical under the existing test suite.
- No regressions in namespace/prolog-oriented tests.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 3: Refactor parser-side utility code in `xmlparse`

Scope:
Tackle the smaller self-contained logic inside `xmlparse.rs` before touching the full parser control flow.

Key files/modules:
- `rexpat/src/lib/xmlparse.rs`

Planned work:
- Refactor local utilities such as siphash helpers, allocation wrappers, and small parser helper routines into idiomatic Rust.
- Replace manual byte assembly, repeated temporary variables, and translated loop scaffolding with slices, local helpers, and standard integer/byte operations where ABI-neutral.
- Document and isolate allocator and callback invariants that must remain tied to the C ABI.

Acceptance criteria:
- Utility/helper sections of `xmlparse.rs` are cleaner and use less broad unsafe code.
- Parser callbacks, allocator behavior, and exported symbols remain unchanged.
- Full test suite still passes.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 4: Refactor `xmltok` scanning primitives and encoding helpers

Scope:
Clean up the tokenizer internals with emphasis on pointer-range handling and repeated token-scanning patterns.

Key files/modules:
- `rexpat/src/lib/xmltok.rs`

Planned work:
- Identify repeated scanner idioms that can be replaced with private helpers over byte ranges or pointer pairs.
- Simplify duplicated token/encoding logic where the generated translation left near-identical branches.
- Reduce full-function unsafe blocks by moving pointer-sensitive operations into narrow helper routines.
- Preserve all exported tokenizer constants, function pointers, and C-facing structs.

Acceptance criteria:
- `xmltok.rs` has less duplicated low-level scanning code and smaller unsafe regions.
- Tokenization behavior remains stable under the parser test suite.
- Any remaining raw-pointer-heavy code has explicit safety rationale.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 5: Refactor the main parser control flow in `xmlparse`

Scope:
Apply the helper patterns from earlier milestones to the main parser logic, focusing on idiomatic control flow and local reasoning.

Key files/modules:
- `rexpat/src/lib/xmlparse.rs`

Planned work:
- Replace translated loop labels, mutable temporary cascades, and broad unsafe regions with structured Rust control flow.
- Encapsulate repeated parser state transitions and buffer management operations behind private helpers.
- Where raw pointers must remain, centralize invariants and minimize aliasing exposure.
- Keep callback ordering, error propagation, and allocation behavior exactly aligned with the C implementation.

Acceptance criteria:
- Main parser flow is materially easier to read and reason about.
- Unsafe code is narrowed and documented at the remaining hard boundaries.
- The full test suite passes without behavioral drift.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 6: Test harness cleanup while preserving C-interface coverage

Scope:
Make the Rust test harness more maintainable without changing the fact that it exercises the library through the C ABI.

Key files/modules:
- `rexpat/src/tests/common.rs`
- `rexpat/src/tests/handlers.rs`
- `rexpat/src/tests/runtests.rs`
- `rexpat/src/tests/*.rs`

Planned work:
- Remove mechanical translation artifacts from the test support code.
- Consolidate duplicated setup/helpers used across the test binaries.
- Keep tests calling the exported C interface rather than importing safe internal Rust APIs.

Acceptance criteria:
- Test code is clearer and less repetitive.
- Test coverage and invocation model remain the same from the perspective of the library ABI.
- `cargo run` continues to pass.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Milestone 7: Final unsafe audit and documentation pass

Scope:
Finish with a focused audit of what unsafe code remains and why it is still required.

Key files/modules:
- `rexpat/lib.rs`
- `rexpat/src/lib/xmlparse.rs`
- `rexpat/src/lib/xmltok.rs`
- `rexpat/src/lib/xmlrole.rs`
- `rexpat/src/tests/*.rs`

Planned work:
- Review all remaining `unsafe` blocks and ensure each one is as small as practical.
- Add or tighten safety comments where the reason is not obvious from nearby code.
- Remove dead code or leftover translation scaffolding exposed by earlier refactors.
- Update this plan with final tradeoffs and any intentionally deferred work.

Acceptance criteria:
- Remaining unsafe code is justified, documented, and concentrated at unavoidable boundaries.
- Formatting, build, and full test suite all pass.
- The crate is left in a state where further safe-Rust migration work has a clear map.

Verification commands:

```bash
cd rexpat
cargo build
cargo fmt --all --check
cargo run
```

## Risk register

1. ABI drift
Mitigation:
Never change exported signatures, symbol names, or `#[repr(C)]` layouts; review any edit touching C-facing types or callbacks against the existing definitions before commit.

2. Pointer provenance and aliasing bugs during cleanup
Mitigation:
Move raw-pointer operations behind narrow private helpers, prefer local slice views only when lifetimes and bounds are explicit, and keep unsafe blocks as small as possible.

3. Behavioral regressions in tokenizer/parser state machines
Mitigation:
Refactor smaller modules first, keep milestones narrow, and run the full `cargo run` test suite after every milestone rather than relying on compile success.

4. Hidden callback-order or allocation-semantics regressions
Mitigation:
Treat allocator wrappers, handler invocation order, and buffer ownership as correctness-critical; document those invariants before rewriting control flow around them.

5. Over-aggressive “safe Rust” conversions that obscure C-compatibility constraints
Mitigation:
Prefer correctness and ABI preservation over stylistic cleanup; keep low-level representations when they are part of the ABI contract, and record tradeoffs here when a more idiomatic option is rejected.

6. Large-file refactors becoming unreviewable
Mitigation:
Stage work by subsystem, commit after each milestone, and avoid mixing helper extraction, control-flow rewrites, and test cleanup in the same patch when they can be separated.

7. Toolchain or formatting drift
Mitigation:
Always run with the pinned nightly toolchain from `rexpat/rust-toolchain.toml`; run `cargo fmt --all --check` on every milestone before commit.

## Tradeoff log

- Initial plan choice: start with `xmlrole` before `xmltok` or the main parser because it is the smallest state-machine module and offers the best chance to establish repeatable cleanup patterns with lower regression risk.
- Initial plan choice: defer test-harness cleanup until the library internals stabilize so tests remain a fixed oracle during the more invasive parser/tokenizer refactors.
