# Variable Shadowing and Scopes

Shadowing via `let x = ...` allocates a new variable binding on the stack rather than mutating existing memory. This permits rebinding to a different type under the same identifier while maintaining immutability across transformations.

## Evidence
Traced in `notes/chapters/deep_dives/scopes_and_shadowing.md` and verified in `tute/rust_book/basics/practice/src/main.rs` (`scope_and_shadowing` function). Differentiated between `let mut x` (same type, mutable in-place) and `let x` (shadowing with potential type changes).

## Implications
Shadowing should be preferred when transforming inputs through distinct processing stages (e.g. converting a raw `&str` buffer into a parsed `usize` length or numeric type) rather than introducing artificial names like `x_str`, `x_parsed`.
