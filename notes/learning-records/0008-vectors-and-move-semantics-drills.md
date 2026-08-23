# Vectors and Move Semantics Drills

Practical execution of move semantics verifies that passing heap-allocated structures (`Vec<T>`) transfers ownership by value unless explicitly borrowed (`&Vec<T>` / `&mut Vec<T>`) or cloned (`.clone()`). Mutability of an owned parameter can be declared at the callee binding (`mut vec: Vec<i32>`), decouple caller immutability from callee mutations without aliasing. Non-Lexical Lifetimes allow sequential mutable borrows as long as their usage spans do not overlap.

## Evidence
- Completed Rustlings drills: `05_vecs` (`vecs1`, `vecs2`) and `06_move_semantics` (`move_semantics1` through `move_semantics5`).
- Progress: 30 / 94 drills completed. Next target: `07_structs`.

## Implications
When designing function interfaces, prefer borrowing (`&[T]`) for read-only inspections, taking by value (`Vec<T>`) when consuming or transforming the buffer, and returning initialized owned buffers directly to leverage move semantics with zero heap allocations.
