# Packages, Crates, and Module Privacy Invariants

A crate is the fundamental atomic unit of compilation in `rustc`. Packages organize crates via `Cargo.toml`. Privacy boundaries enforce encapsulation: parent modules cannot access child internals, struct fields remain private by default unless marked `pub` (requiring constructor functions for invariant protection), and enum variants are public by default to ensure match exhaustiveness.

## Evidence
- Completed Chapter 7 of TRPL:
  - 7.1: Packages and Crates (Binary vs. Library crates, crate roots).
  - 7.2: Defining Modules and the Module Tree hierarchy.
  - 7.3: Absolute (`crate::`) vs Relative (`super::`) path resolution.
  - 7.4: `use` keyword bindings, disambiguation via `as`, and re-exporting with `pub use`.
  - 7.5: Modern multi-file module architecture (2018/2024 edition).
- Implemented and verified code in [`tute/rust_book/restaurent/`](file:///home/bhondu/coding/rust/tute/rust_book/restaurent/):
  - [`src/lib.rs`](file:///home/bhondu/coding/rust/tute/rust_book/restaurent/src/lib.rs)
  - [`src/front_of_house.rs`](file:///home/bhondu/coding/rust/tute/rust_book/restaurent/src/front_of_house.rs)
  - [`src/front_of_house/hosting.rs`](file:///home/bhondu/coding/rust/tute/rust_book/restaurent/src/front_of_house/hosting.rs)
  - [`src/front_of_house/serving.rs`](file:///home/bhondu/coding/rust/tute/rust_book/restaurent/src/front_of_house/serving.rs)
- Authored Chapter 7 master notes in [`notes/chapters/ch07_packages_crates_modules.md`](file:///home/bhondu/coding/rust/notes/chapters/ch07_packages_crates_modules.md).

## Implications
- In Yapbook: Structure `src-tauri/src/` into a clean multi-file module tree (`models/`, `parser/`, `analytics/`, `commands/`), using `pub(crate)` to keep internal invariants safe while exposing clean Tauri command entry points.
