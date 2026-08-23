# Algebraic Data Types and Pattern Matching

Rust enums are tagged unions where each variant can encapsulate distinct payload types. The compiler enforces exhaustive pattern matching across all variants at compile time, eliminating unhandled runtime states.

## Evidence
Documented in `notes/chapters/deep_dives/enums_and_unions.md` and implemented in `tute/rust_book/guessing_game/src/main.rs` using `std::cmp::Ordering` (`Less`, `Equal`, `Greater`) and `Result` (`Ok(num)`, `Err(_)`).

## Implications
State machines and multi-variant domain types should be encoded using enums rather than boolean flags or loose structs. Adding new variants will systematically flag every unhandled callsite across the codebase at compile time.
