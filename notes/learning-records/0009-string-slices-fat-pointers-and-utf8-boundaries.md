# String Slices, Fat Pointers, and UTF-8 Boundary Invariants

String slices (`&str`) and generic slices (`&[T]`) are 16-byte fat pointers (`ptr` + `len`) that provide zero-copy, non-owning windowing into contiguous memory across the heap, stack, or `.rodata` static sections. Slices eliminate state desynchronization bugs by tying slice validity directly to the owner's borrow lifecycle. Slicing string slices operates on raw byte offsets, requiring adherence to multi-byte UTF-8 character boundaries to avoid runtime panics.

## Evidence
- Authored Chapter 4.3 master notes in [`notes/chapters/ch04_03_slices.md`](file:///home/bhondu/coding/rust/notes/chapters/ch04_03_slices.md).
- Analyzed the memory differences between thin pointers (`&String`), fat pointers (`&str`, `&[T]`), and owned dynamic vectors/strings.
- Formulated the state desynchronization problem (`first_word`) and proven compile-time prevention via Aliasing XOR Mutability.
- Integrated UTF-8 character boundary analysis and `.chars()` / `.char_indices()` safe traversal patterns.

## Implications
Zero-copy streaming parsers (e.g. `stream-slice`) and network protocol handlers must accept `&[u8]` and `&str` to process incoming buffers directly in-place without heap reallocation or defensive cloning.
