# Rustlings Drills: Strings, HashMaps, Options, and Error Composition

Comprehensive consolidation of Rustlings problem sets spanning Strings (`09_strings`), Modules (`10_modules`), HashMaps (`11_hashmaps`), Quiz 2 (`quizzes/quiz2.rs`), Options (`12_options`), and Error Handling (`13_error_handling`).

## Evidence & Solved Drills (55 / 96 Drills Completed)
- **`09_strings` (4/4)**:
  - Dissected allocation costs of string methods: zero-cost views (`&str`, `.trim()`, byte range slicing `&s[0..1]`) vs heap-allocating transformations (`.to_string()`, `.to_owned()`, `.into()`, `format!`, `.replace()`, `.to_lowercase()`).
- **`10_modules` (3/3)**:
  - Configured module visibility trees, resolved nested namespaces with `pub use`, and aliased overlapping symbols.
- **`11_hashmaps` (3/3)**:
  - Implemented multi-field structural aggregations using the `Entry` API (`*map.entry(team).or_insert(TeamStats::new())`) to track match scores without duplicate lookups.
- **`quizzes/quiz2.rs` (1/1)**:
  - Constructed string transformation state machine. Proved memory superiority of in-place `s.push_str("bar")` loops over `"bar".repeat(n)` by eliminating intermediate throwaway heap allocations.
- **`12_options` (3/3)**:
  - Formulated inclusive range patterns in `match` (`0..=21`), tested option extraction via `.unwrap()`, and prevented use-after-move errors (`E0382`) on non-`Copy` inner structs using match ergonomics (`match &optional_point`) and `ref` pattern bindings.
- **`13_error_handling` (6/6)**:
  - Evaluated 4 error handling styles: `?` operator, functional `Result::map`, explicit `match`, and `let else` guard clauses.
  - Implemented composite error tagged unions (`ParsePosNonzeroError`) bridging disjoint subsystem errors (`ParseIntError`, `CreationError`) via `.map_err()` and `impl From<T> for E` trait implementations.

## Implications
- In Yapbook: Structure the multi-stage parser with custom error enums and `From` trait implementations, allowing message timestamp tokenization, regex splitting, and database transactions to propagate errors with clean `?` syntax.
