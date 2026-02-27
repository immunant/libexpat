# libexpat

- The `src` folder contains a Rust port of the C code in the `expat` folder.
- Do not touch any C sources or headers under `expat/`.
- Work only on Rust-side files (`src/`, `lib.rs`, `build.rs`, `Cargo.toml`, and other Rust tooling files).

# Development workflow
- Each commit should
    + be self-contained with a clearly defined purpose
    + make the Rust code more idiomatic, for example, it might:
        + replace C types or Rust equivalents,
        + change C-style control flow to Rust-style control flow
    + keep FFI/ABI compatibility with the C library:
        + keep `extern "C"` signatures and `#[no_mangle]` exported symbol names stable,
        + preserve `#[repr(C)]` layouts and field order for C-facing types,
        + avoid unwinding across FFI boundaries
    + pass the test suite,
    + pass formatting checks
- Each commit message should clearly summarize the changes made.

## Build

Use the pinned toolchain from `rust-toolchain.toml` (`nightly-2025-08-01`).

To build the project:

```bash
cargo build
```

## Formatting

Formatting is required per commit:

```bash
cargo fmt --all --check
```

## Testing

###  Functional correctness

To run the test suite:

```bash
./test.sh r
```

For behavioral comparison/debugging against the C implementation:

```bash
./test.sh c
```
