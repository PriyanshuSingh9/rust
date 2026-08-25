# Enums, Tagged Unions, and Exhaustive Pattern Matching

Enums represent algebraic sum types ($A + B$) where values occupy overlapping stack space equal to the largest variant payload plus an alignment-padded discriminant tag. The compiler guarantees memory safety by enforcing compile-time match exhaustiveness and applying niche optimizations (e.g. zero-overhead `Option<&T>` utilizing null-pointer niches).

## Evidence
- Completed Chapter 6 of TRPL:
  - 6.1: Defining Enums, payload attachments, and `impl` methods.
  - 6.2: `Option<T>` null-pointer elimination and reference push-down mechanics (`&Option<T>` -> `Option<&T>`).
  - 6.3: Exhaustive `match` expressions, payload bindings, `if let`, and `let else` control flow.
- Implemented and verified code in [`tute/rust_book/enums/src/main.rs`](file:///home/bhondu/coding/rust/tute/rust_book/enums/src/main.rs).
- Authored Chapter 6 master notes in [`notes/chapters/ch06_enums.md`](file:///home/bhondu/coding/rust/notes/chapters/ch06_enums.md).
- Formally analyzed memory differences between Product Types (`struct`, $\sum \text{size}$) and Sum Types (`enum`, $\max(\text{size}) + \text{tag}$).

## Implications
- In Yapbook: Model domain entities and errors as algebraic sum types (`MessageType::User`, `MessageType::System`, `AppError::Io`, `AppError::Parse`) to make impossible states unrepresentable and eliminate runtime null checks.
- Use `let else` guard clauses to simplify nested parser branches in the WhatsApp stream processor.
