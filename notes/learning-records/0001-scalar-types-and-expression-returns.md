# Scalar Types and Expression Returns

Statements perform actions and discard results, returning the unit type `()`. Expressions evaluate to values. Appending a trailing semicolon to the final line of a function or block converts an expression into a statement, yielding `()` and causing type mismatch error `E0308`.

## Evidence
Observed in `tute/rust_book/basics/functions/src/main.rs` and `tute/rust_book/basics/practice/src/main.rs`. Implemented `compute_grade` and `iter_fibonnacci` using bare tail expressions for function returns.

## Implications
All control flow branches (`if/else`, `match`) must evaluate to identical types without accidental trailing semicolons on branch return values.
