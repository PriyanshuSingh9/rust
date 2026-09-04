# Traits, Coherence, and Blanket Implementations

Traits define shared contracts in Rust, decoupled from concrete struct layout. The compiler enforces the coherence invariant via the orphan rule, preventing global namespace collisions by requiring either the trait or the target type to be local to the declaring crate. Default methods can delegate to un-implemented required methods. The `impl Trait` parameter syntax provides sugar over trait bounds, while `impl Trait` return types strictly mandate a single concrete type across all branches due to static monomorphization. Trait bounds permit conditional method implementation and blanket implementations across entire families of types.

## Evidence
- Completed Chapter 10.2 of TRPL:
  - Trait declaration and implementation (`pub trait Summary`, `NewsArticle`, `SocialPost`).
  - Coherence and the orphan rule invariants.
  - Method delegation in default implementations (`summarize_author` calling `self.author()`).
  - Multiple trait bounds (`+`), `where` clauses, and the semantic difference between `&impl Trait` and `<T: Trait>`.
  - Compile-time single concrete type requirement for `-> impl Trait`.
  - Conditional method implementations on `Pair<T: Display + PartialOrd>`.
  - Blanket implementations (`impl<T: Display> ToString for T`).
- Implemented and verified clean execution in [`tute/rust_book/traits/src/lib.rs`](file:///home/bhondu/coding/rust/tute/rust_book/traits/src/lib.rs) and [`tute/rust_book/traits/src/main.rs`](file:///home/bhondu/coding/rust/tute/rust_book/traits/src/main.rs).
- Authored Chapter 10.2 master notes in [`notes/chapters/ch10_02_traits.md`](file:///home/bhondu/coding/rust/notes/chapters/ch10_02_traits.md).

## Implications
- In Yapbook: Define domain traits like `MessageParser` and `AnalyticsAggregator`. Use blanket implementations (`impl<T: MessageParser> StreamProcessor for T`) to automatically wire serialization, checksum verification, and SQLite persistence pipelines across all parser variants with zero boilerplate.
