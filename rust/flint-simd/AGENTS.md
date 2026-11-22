# AGENTS.md

This file provides guidance for agentic coding agents (AI assistants, automated tools) working on
the `flint-simd` crate.

---

## Where this crate lives

`flint-simd` is one part of the `flint` monorepo (`github.com/jefwagner/flint`):

```
flint/                      # repo root
├── src/flint/, tests/,     # numpy-flint: the C extension published to PyPI
│   docs/, pyproject.toml   #   (docs deploy to jefwagner.github.io/flint)
├── rust/                   # Cargo workspace root
│   ├── Cargo.toml          #   members = ["flint-simd"]
│   ├── rust-toolchain.toml #   pins nightly
│   └── flint-simd/         #   ← this crate
└── legacy/flint-rs/        # superseded PyO3 prototype; not in the workspace, do not modify
```

Only touch files outside `rust/` when the task explicitly calls for it.

---

## Project Overview

`flint-simd` (package name `flint` in `Cargo.toml`) is a rounded floating-point interval
arithmetic library. The basic type `Flint<T>` holds a lower and upper bound (`lb`, `ub`) for
`f32`/`f64`, and the exact result of a computation always lies within that interval.

To support SIMD, array types use a struct-of-arrays layout (separate `lb` and `ub` arrays)
instead of an array of `Flint` structs. The public types are:

| Type | Kind |
|------|------|
| `Flint<T>` | Owned scalar (`Copy`) |
| `FlintRef<'a, T>` / `FlintMut<'a, T>` | Borrowed scalar (shared / mutable) |
| `FlintArray<T, N>` | Owned fixed-size array; `N` encodes shape (e.g. 16 for a 4×4 matrix) |
| `FlintVec<T>` | Owned heap-allocated array |
| `FlintView<'a, T>` / `FlintViewMut<'a, T>` | Borrowed slice of an array or vec |

`FlintSoA` / `FlintSoAMut` are the shared traits over the array-like types. No type carries
shape or stride metadata; multi-dimensional meaning comes from `N` and the methods built on it.

The design rationale and the TODO list are in `notes.md`.

---

## Nightly Rust

The crate uses two nightly-only features and **will not compile on stable Rust**:

```rust
#![feature(portable_simd)]      // std::simd
#![feature(macro_metavar_expr)] // ${count(...)}, ${index()} in declarative macros
```

`rust/rust-toolchain.toml` selects nightly automatically, so run plain `cargo` commands from
`rust/` or any directory below it. No `+nightly` is needed.

---

## Build, Check, Lint, and Test Commands

Run from `rust/` (the workspace root) or from `rust/flint-simd/`.

```bash
cargo build
cargo check            # fast, no linking
cargo clippy           # lint
cargo fmt              # format
cargo test             # all tests
cargo doc --open       # API docs
```

Run a subset of tests by passing a substring of the test path:

```bash
cargo test f32_nu_nd               # next_up_down::test::f32_nu_nd
cargo test test_equality_flint     # cmp::test::test_equality_flint
cargo test cmp::                   # every test in a module
```

Current baseline: `cargo test` passes, but `cargo clippy` reports a few warnings and the tree is
not yet `cargo fmt`-clean. Don't mix bulk reformatting or lint cleanups into unrelated changes.

---

## Crate Layout

```
rust/flint-simd/
├── Cargo.toml          # edition 2024; deps: num-traits, ryu
├── notes.md            # design notes + TODO list (checkbox style)
└── src/
    ├── lib.rs          # crate root: feature flags, type definitions, SoA traits
    ├── next_up_down.rs # NextUpDown trait: scalar, SIMD, array, Vec impls
    ├── fmt.rs          # Display/Debug (shortest number inside the interval)
    ├── conv.rs         # From/Into conversions + creation macros
    ├── cmp.rs          # PartialEq / PartialOrd
    ├── arith.rs        # arithmetic operators, scalar/array broadcasting
    ├── linalg.rs       # 4×4 linear algebra (IDENTITY_4X4, etc.)
    └── math.rs         # elementary math functions
```

---

## Code Style

### Formatting

- Rust edition **2024**, `rustfmt` defaults (there is no `rustfmt.toml`).
- 4-space indentation; no tabs.
- Trailing commas in multi-line struct literals, function arguments, and match arms.
- Put `where` clauses on their own line when the trait bounds are complex or numerous.

### Comments

- Use `///` doc comments for all public items (types, traits, methods, macros, constants).
- Use `//` line comments for inline notes; avoid `/* */` block comments.
- Separate sections inside large `impl` blocks with `// ----` dash lines.

### Imports

- Order: `std`/core first, then external crates, then `crate::` imports.
- Use `use std::simd::prelude::*;` for SIMD and import any other SIMD items one by one
  (e.g. `use std::simd::Select;`).
- Test submodules bring in the parent's items with `use super::*;`.
- Import external crates (`num_traits`, `ryu`) directly, without aliases.
- Use `#[allow(unused_imports)]` only for items that are needed solely by a macro expansion.

### Naming

| Construct | Convention | Examples |
|-----------|-----------|---------|
| Types / structs / traits | `PascalCase` | `Flint`, `FlintArray`, `NextUpDown` |
| Methods / functions | `snake_case` | `next_up`, `as_ref`, `from_str_parts` |
| Constants | `SCREAMING_SNAKE_CASE` | `F32_SIGN_MASK`, `IDENTITY_4X4` |
| Macros | `snake_case!` | `flint32!`, `flint64!`, `impl_small_int!` |
| Type parameters | Single uppercase letter | `T` (float type), `N` (lane/element count) |
| Struct fields | `snake_case`, private | `lb`, `ub` |

The abbreviations `lb` (lower bound) and `ub` (upper bound) are canonical. Use them
consistently instead of the full words.

---

## Error Handling

- This is a **pure numeric library**: the public API never fails, and it panics only on
  programmer error.
- Use `.expect("descriptive message")` in internal helpers where a failure means there is a bug.
  Never use bare `unwrap()` in library code.
- Use `unsafe { std::str::from_utf8_unchecked(...) }` only for buffers known to be ASCII, with a
  comment explaining why that holds.
- `partial_cmp` returns `None` whenever either operand has a `NaN` bound. Never silently drop
  NaN information.
- The public API has no `Result` types yet. For future fallible operations, use `Option<T>` when
  a value can't be represented and `Result<T, E>` for parsing or I/O.

---

## Testing

- Tests are inline `#[cfg(test)] mod test { ... }` blocks at the bottom of each source file.
  There are no integration test files yet.
- `lib.rs` and `fmt.rs` use `mod tests`. Prefer `mod test` in new files.
- Use plain `assert_eq!` / `assert!` / `assert_ne!`, with no external test framework.
- Each test function checks one behaviour and is named `test_<what_is_being_tested>`.

---

## Patterns

- **Generic floats:** generic `impl`s usually need `T: num_traits::Float + Copy + ...`. Add only
  the bounds the body actually uses.
- **SIMD:** SIMD code targets `std::simd`, with lane counts as `const` generics. Local aliases
  such as `SIMDf32xN = Simd<f32, N>` make `impl` blocks easier to read.
- **Macros:** prefer `macro_rules!` over proc macros for repetitive `impl`s, using
  `${count(...)}` / `${index()}` to avoid manual indexing.
- **TODO tracking:** keep the checkbox list in `notes.md` up to date (`[x]` done, `[_]` pending).

---

## Things to Avoid

- Don't modify `legacy/flint-rs/`, and don't add it to the workspace.
- Don't add `unwrap()`, `std::process::exit`, or panics reachable from the public API.
- Don't add dependencies without checking whether they support `no_std`, since `no_std` may
  become a goal (see `notes.md`).
