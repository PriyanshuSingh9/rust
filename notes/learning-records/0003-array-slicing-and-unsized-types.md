# Array Slicing, Unsized Types, and Fat Pointers

Range indexing on an array `a[1..4]` yields an unsized slice `[T]`, not a fixed-size array `[T; 3]`. Because `[T]` has a dynamically determined size at runtime, it cannot exist as a bare local variable on the stack.

## Evidence
Detailed in `notes/chapters/deep_dives/array_slicing.md` and implemented in `tute/rust_book/basics/slices_and_copies/src/main.rs`. Demonstrated that `&a[1..4]` produces a sized 16-byte fat pointer `(ptr, len)`. Owned duplicates require explicit `.to_vec()` or buffer copies with `.copy_from_slice()`.

## Implications
Functions consuming contiguous data should accept slice references `&[T]` instead of concrete containers like `&[T; N]` or `&Vec<T>`, maximizing caller compatibility.
