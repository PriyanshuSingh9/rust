# Common Collections, Pointer Invalidation, and UTF-8 Boundaries

Dynamic standard collections (`Vec<T>`, `String`, `HashMap<K, V>`) are tri-word stack structures (`ptr`, `cap`, `len`) managing heap-allocated memory. Rust's borrow checker prevents pointer invalidation bugs (`E0502`) during vector reallocations, strictly forbids arbitrary string integer indexing to guarantee $O(1)$ performance and UTF-8 validity, and provides the single-lookup `Entry` API for hash tables.

## Evidence
- Completed Chapter 8 of TRPL:
  - 8.1: Vectors (`Vec<T>`, heap capacity allocation, safe `.get()` access, enum wrapping for heterogeneous collections).
  - 8.2: UTF-8 Strings (`String` vs `&str`, byte length variance, `.bytes()` vs `.chars()` vs grapheme clusters, concatenation via Deref coercion).
  - 8.3: Hash Maps (`HashMap<K, V>`, ownership transfers, `Entry` API with `.or_insert()`, SipHash collision protection).
- Authored Chapter 8 master notes in [`notes/chapters/ch08_common_collections.md`](file:///home/bhondu/coding/rust/notes/chapters/ch08_common_collections.md).
- Formally modeled pointer invalidation scenarios during capacity growth and the three representations of Unicode strings.

## Implications
- In Yapbook: Use `HashMap<&str, ParticipantStats>` with the `Entry` API to implement the single-pass $O(N)$ analytics engine, computing message volumes and reply graphs across 126k messages in < 500 ms without redundant lookups.
- In stream parsing: Process WhatsApp chat lines as `&str` slices and iterate with `.char_indices()` to respect multi-byte UTF-8 boundaries during tokenization.
