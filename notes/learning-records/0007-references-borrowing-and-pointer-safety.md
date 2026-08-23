# References, Borrowing, and the Pointer Safety Principle

Pointers enable aliasing (multiple paths to the same memory). While read-only aliasing is benign, aliasing combined with mutation leads to Use-After-Free via deallocation, pointer/iterator invalidation via heap buffer reallocation, and data races via unsynchronized concurrency. Rust eliminates these failure modes using the Borrow Checker's static permission model (Read, Write, Own), Non-Lexical Lifetimes (NLL), and the Aliasing XOR Mutability invariant.

## Evidence
- Authored Chapter 4.2 master notes in [`notes/chapters/ch04_02_references_and_borrowing.md`](file:///home/bhondu/coding/rust/notes/chapters/ch04_02_references_and_borrowing.md).
- Traced the physical memory mechanics of `Vec::push` reallocation and pointer invalidation.
- Formalized the Brown R-W-O permission tracking lifecycle for immutable (`&T`) and mutable (`&mut T`) borrows.
- Updated definitions for `Pointer Safety Principle`, `Aliasing`, `Pointer / Iterator Invalidation`, and `Dangling Reference` in [`notes/GLOSSARY.md`](file:///home/bhondu/coding/rust/notes/GLOSSARY.md).

## Implications
High-performance systems architectures (e.g. zero-copy parsers and ring buffers) must structure memory access around non-lexical borrow boundaries to prevent heap reallocation invalidations without introducing defensive memory clones.
