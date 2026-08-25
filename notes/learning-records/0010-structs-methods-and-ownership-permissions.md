# Structs, Methods, and Ownership Permissions

Structs group heterogeneous fields into contiguous stack-allocated compound types. Methods declared within `impl` blocks operate under the compile-time permissions model: `&self` requires Read (`R`), `&mut self` requires Read and Write (`R + W`), and `self` by-value requires Read and Own (`R + O`). Moving out of a reference (`*self`) is rejected by the borrow checker (`E0507`) to eliminate use-after-free and double-free vulnerabilities for heap-allocated types unless the struct explicitly implements `Copy`.

## Evidence
- Completed Chapter 5 of TRPL:
  - 5.1: Defining and Instantiating Structs (named-field, tuple structs, unit-like structs).
  - 5.2: Example Program Using Structs (`#[derive(Debug)]`, `dbg!` macro, stderr logging).
  - 5.3: Method Syntax & Ownership (methods, associated functions, constructors, `impl` blocks).
- Implemented comprehensive verified examples in [`tute/rust_book/structs/src/main.rs`](file:///home/bhondu/coding/rust/tute/rust_book/structs/src/main.rs).
- Authored Chapter 5 master notes in [`notes/chapters/ch05_structs.md`](file:///home/bhondu/coding/rust/notes/chapters/ch05_structs.md).
- Authored Rust Attributes and Derives reference in [`notes/reference/attributes.md`](file:///home/bhondu/coding/rust/notes/reference/attributes.md).
- Proved double-free prevention during moves behind `&mut self` and established explicit `Copy` derivation rationale.

## Implications
When designing domain types and network protocol state machines:
- Use `&self` for non-destructive inspections.
- Use `&mut self` for state transitions.
- Use `self` for terminal transitions that consume or convert the struct (e.g. builder pattern to final immutable connection), preventing stale states at compile time.
