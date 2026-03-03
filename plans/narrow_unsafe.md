# Plan: Narrow Rust `unsafe` Blocks to Smallest Expressions

## Baseline Inventory

- Total `unsafe { ... }` blocks in Rust sources: `438`
- Already single-line expression blocks: `66`
- By file (total / single-line):
  - `src/lib/xmlparse.rs`: `172 / 4`
  - `src/lib/xmltok.rs`: `114 / 15`
  - `src/lib/xmlrole.rs`: `55 / 1`
  - `src/xmlwf/xmlwf.rs`: `44 / 4`
  - `src/xmlwf/xmlfile.rs`: `32 / 24`
  - `src/xmlwf/readfilemap.rs`: `21 / 18`

## Scope and Constraints

- Edit Rust-side files only (`src/`, `lib.rs`, `build.rs`, `Cargo.toml`, tooling files).
- Do not touch `expat/` C headers/sources.
- Preserve ABI/FFI compatibility:
  - Keep `extern "C"` signatures and `#[no_mangle]` symbol names stable.
  - Keep `#[repr(C)]` layouts and field order unchanged.
  - Avoid unwinding across FFI boundaries.

## Unsafe-Narrowing Rules

- Target form: one `unsafe` operation per `unsafe { ... }` expression.
- Keep control flow (`if`, `match`, loops) outside `unsafe`.
- Move safe arithmetic and data-flow out of `unsafe`.
- Use small helper functions only when they reduce repetition without hiding safety assumptions.
- Do not widen aliasing guarantees (avoid converting raw pointers to references unless proven sound).

## Execution Plan (Commit-Sized)

1. **Establish progress tracking**
   - Add repeatable local metrics commands for:
     - total `unsafe` block count
     - single-line `unsafe` expression count
   - Use the same metrics after each commit.

2. **Low-risk cleanup first (`xmlwf` file I/O helpers)**
   - Refactor `src/xmlwf/xmlfile.rs` and `src/xmlwf/readfilemap.rs`.
   - Most unsafe already expression-level; finish remaining broad blocks.
   - Keep behavior byte-for-byte equivalent.

3. **Refactor `src/xmlwf/xmlwf.rs`**
   - Split mixed `unsafe` blocks so only pointer/FFI operations remain unsafe.
   - Keep parser driver logic and branching fully safe.

4. **Refactor `src/lib/xmlrole.rs`**
   - Convert parser state-machine routines from broad `unsafe` regions to micro unsafe expressions.
   - Extract repeated pointer reads/writes into tiny helpers only where it improves clarity.

5. **Refactor `src/lib/xmltok.rs` in two passes**
   - Pass A: casting/slice/conversion helper areas.
   - Pass B: scanner/tokenizer routines with broad `unsafe` sections.
   - Ensure tokenization behavior remains unchanged.

6. **Refactor `src/lib/xmlparse.rs` in two passes**
   - Pass A: utility sections (including siphash-related code).
   - Pass B: parser core/state transitions/callback plumbing.
   - Keep all exported and callback-facing contracts unchanged.

## Validation Gate (Per Commit)

Run with pinned toolchain `nightly-2023-04-15`:

```bash
cargo +nightly-2023-04-15 build
cargo +nightly-2023-04-15 fmt --all --check
./test.sh r
```

Additional comparison when touching parser/tokenizer behavior:

```bash
./test.sh c
```

## Completion Criteria

- No broad multi-statement `unsafe` blocks remain where a smaller expression form is possible.
- Remaining `unsafe` usage is local and operation-specific.
- Formatting and tests pass on the pinned toolchain.
- Commit history remains self-contained and purpose-driven.
