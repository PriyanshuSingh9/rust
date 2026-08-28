# Error Handling, The `?` Operator, and Invariant Types

Rust treats error handling as a bifurcated systems discipline: unrecoverable invariant violations trigger deterministic stack unwinding via `panic!`, while expected domain failures are encoded into `Result<T, E>`. The `?` operator performs asymmetric error propagation, leveraging `From::from` coercion to unify disparate subsystem errors while early-returning on failure and unwrapping on success.

## Evidence
- Completed Chapter 9 of TRPL:
  - 9.1: Unrecoverable errors (`panic!`, backtraces with `RUST_BACKTRACE=1`, unwinding vs abort).
  - 9.2: Recoverable errors with `Result<T, E>`, `ErrorKind` branching, `unwrap_or_else`, propagation via `?`, `From` trait coercion, `fs::read_to_string`, and `main() -> Result<(), Box<dyn Error>>`.
  - 9.3: To `panic!` or not to `panic!`, human invariants vs compiler checks, and custom invariant types (`Guess` struct pattern).
- Implemented and verified clean execution in [`tute/rust_book/error_handling/src/main.rs`](file:///home/bhondu/coding/rust/tute/rust_book/error_handling/src/main.rs).
- Authored Chapter 9 master notes in [`notes/chapters/ch09_error_handling.md`](file:///home/bhondu/coding/rust/notes/chapters/ch09_error_handling.md).

## Implications
- In Yapbook: Structure the global `AppError` enum using `thiserror` (with `#[from]` annotations for `io::Error` and `serde_json::Error`), allowing IPC commands and SQLite writer actors to propagate failures with clean `?` syntax.
- Use newtype validation structs to encapsulate invariants for message IDs, timestamps, and session tokens before dispatching to analytics engines.
