# Rust Systems & Distributed Systems Glossary

Canonical terminology and precise definitions for this workspace.

---

## Memory, Layout & Types

**Scalar Type**:
A primitive type representing a single value (integers, floating-point numbers, booleans, and characters).

**Compound Type**:
A type that groups multiple values into one structure (fixed-size tuples and fixed-size arrays).

**Fat Pointer**:
A two-word pointer on the stack containing both a memory address and metadata (such as length for slices `&[T]` or a vtable pointer for trait objects `&dyn Trait`).

**Dynamically Sized Type (DST) / Unsized Type**:
A type whose size cannot be determined at compile time (such as `[T]` or `str`). Sized variables cannot hold DSTs directly on the stack; they must exist behind a pointer (`&[T]`, `Box<str>`).

**Move Semantics**:
The transfer of ownership of a resource from one variable binding to another via a shallow bitwise copy of its stack descriptor and compile-time invalidation of the source binding.

**Non-Lexical Lifetimes (NLL)**:
A borrow-checker mechanism that calculates liveness of references based on control-flow graph usage rather than curly-brace lexical scopes.

**Interior Mutability**:
A design pattern in Rust that allows mutating data even when there are immutable references to that data, enforcing borrow rules at runtime (`RefCell<T>`) or via synchronization primitives (`Mutex<T>`, `RwLock<T>`).

**Monomorphization**:
The compile-time process of turning generic code into specific code by generating copies of functions for each concrete type used, enabling static dispatch with zero runtime performance cost.

**Dynamic Dispatch (`dyn Trait`)**:
Indirect method invocation resolved at runtime via a vtable pointer inside a fat pointer (`[data_ptr, vtable_ptr]`), enabling heterogeneous collections at the cost of branch prediction overhead and inhibiting compiler inlining.

---

## Hardware Architecture & Memory Safety

**Memory Management Unit (MMU)**:
A hardware component on the CPU that translates virtual addresses into physical DRAM addresses using multi-level page tables and caches translations in the TLB (Translation Lookaside Buffer).

**Segmentation Fault (`SIGSEGV`)**:
An operating system signal (signal 11) sent to a process when hardware memory protection is violated, such as dereferencing an unmapped virtual address, writing to read-only memory, or crossing into kernel space.

**Page Fault (`#PF`)**:
A CPU hardware interrupt (Vector 14) raised when an instruction references a virtual page that is either not mapped in physical RAM (Present bit = 0) or lacks the required access permissions (R/W/X).

**Stack Guard Page**:
An unallocated, protected page with `PROT_NONE` permissions placed at the bottom of a thread's stack region by the OS kernel to detect stack overflows and trigger an instant abort before adjacent memory can be corrupted.

**Undefined Behavior (UB)**:
A state in which code breaks language specification invariants, freeing optimizing compilers to make aggressive assumptions (e.g. deleting null checks, removing loops, or reordering instructions) that result in unpredictable execution, memory corruption, or security vulnerabilities.

**Use-After-Free (UAF)**:
A memory safety flaw occurring when a program accesses memory via a pointer after that memory has been deallocated, allowing malicious payloads to hijack control flow upon reallocation.

**Double Free**:
A memory corruption vulnerability where deallocation (`free()`) is invoked twice on the same memory address, corrupting allocator metadata and freelist links.

**Pointer Safety Principle**:
The architectural rule that data must never be simultaneously aliased and mutated. Enforced in Rust via the Aliasing XOR Mutability invariant.

**Aliasing**:
Accessing the same memory location through multiple different variable bindings or pointer paths. Harmless when read-only; catastrophic when combined with mutation or deallocation.

**Pointer / Iterator Invalidation**:
A bug where modifying or growing a data structure causes its internal memory buffer to be reallocated, turning existing pointers or references into dangling pointers pointing to freed memory.

**Dangling Reference**:
A reference that points to invalid memory (e.g. stack memory whose frame has been popped upon function return, or heap memory that has been deallocated). Prevented in Rust at compile time.

---

## Language Semantics & Patterns

**Statement**:
An instruction that performs an action and does not return a value. Statements end with semicolons.

**Expression**:
A code segment that computes and evaluates to a resultant value. Expressions do not end with semicolons when returned from a block.

**Shadowing**:
Re-declaring a variable using the `let` keyword with an existing identifier. This allocates a new variable binding, permits changing the type, and keeps the binding immutable unless explicitly marked `mut`.

**Algebraic Data Type (ADT)**:
A composite type where values can be formed by sum types (enums) or product types (structs/tuples). Rust enums can carry distinct payload types per variant.

**RAII (Resource Acquisition Is Initialization) / Drop**:
A deterministic resource cleanup pattern where resources are freed immediately when their owning scope ends via the `Drop` trait.

**Type-State Pattern**:
A idiom encoding state machine invariants into distinct Rust types (`DraftPost -> PendingReviewPost -> Post`). Transitions consume `self` by value, making invalid operations impossible to represent at compile time.

**Associated Function**:
A function defined within an `impl` block that does not accept a `self` parameter (e.g., `String::from`, `Rectangle::new`). Namespaced to the type and invoked with `Type::function()`, commonly used as constructors.

**Automatic Referencing and Dereferencing**:
A compiler ergonomics feature where method invocation syntax (`value.method()`) automatically adds `&`, `&mut`, or `*` to match the method's receiver signature without requiring explicit pointer dereferences.

**Attribute**:
Compile-time metadata attached to items or crates using outer (`#[...]`) or inner (`#![...]`) syntax to instruct the compiler on code generation (`derive`), linting (`allow`/`deny`), conditional compilation (`cfg`), memory layout (`repr`), or testing (`test`).

**Tagged Union / Discriminant**:
A data structure representation of an enum consisting of an integer tag (discriminant) identifying the active variant alongside an overlapping memory payload sized to the largest variant.

**Niche Optimization (Null-Pointer Optimization)**:
A compiler layout optimization where enum discriminant tags are omitted if a variant payload contains invalid bit patterns (niches, such as `0x0` for `&T` or `NonNull<T>`), achieving zero memory overhead for `Option<&T>`.

**Crate Root**:
The source file from which `rustc` starts compiling a crate (`src/lib.rs` for library crates, `src/main.rs` for binary crates). Forms the root module referenced as `crate::`.

**Re-exporting (`pub use`)**:
An idiom bringing an item into scope and simultaneously making it public to external consumers, allowing library authors to present a clean public API that differs from the internal module file layout.

**Module Tree**:
The hierarchical tree structure formed by modules and submodules inside a crate that controls item namespacing and privacy boundaries.

**Deref Coercion**:
An automatic compiler conversion that converts a reference to a type implementing the `Deref` trait into a reference to its target type (e.g., `&String` automatically coercing to `&str`, `&Vec<T>` to `&[T]`).

**Entry API**:
A standard library `HashMap` and `BTreeMap` interface (`.entry(key)`) that performs a single lookup to inspect, insert, or mutate key-value pairs in place without redundant hash recalculations.

**SipHash**:
A cryptographic, collision-resistant pseudo-random hashing algorithm used as the default hasher in Rust's standard library `HashMap` to prevent HashDoS denial-of-service vulnerabilities.

**Grapheme Cluster**:
A user-perceived character consisting of one or more Unicode scalar values combined together (such as a base character plus combining accent marks, or multi-byte ligature sequences).

**`ref` Keyword (Pattern Binding)**:
A pattern matching modifier used to bind a variable by reference (`&T` or `&mut T`) instead of taking ownership by value, preventing moves on non-`Copy` types during struct and enum destructuring.

**Match Ergonomics (RFC 2005)**:
A compiler enhancement (Rust 2018+) that automatically infers reference bindings (`&` / `&mut`) when matching on borrowed references (`match &val`), largely replacing manual `ref` annotations.

**`map_err` Combinator**:
A functional method on `Result<T, E1>` that converts the error variant `Err(E1)` into `Err(E2)` via a mapping closure `FnOnce(E1) -> E2`, commonly used to adapt foreign subsystem errors into a crate's unified domain error type.

**Monomorphization**:
The compile-time process of turning generic code into specialized concrete machine code by generating distinct functions and data structures for each concrete type combination used, ensuring zero runtime overhead.

**Static Dispatch**:
Resolving function and method calls at compile time so the CPU directly jumps to known instruction addresses without runtime virtual method table (`vtable`) lookups or pointer indirection.

**Trait Bound**:
A constraint applied to a generic type parameter (e.g., `T: PartialOrd + Display`) that restricts `T` to types implementing specific traits, granting the generic code permission to invoke those traits' methods.

**Coherence (The Orphan Rule)**:
A core language invariant requiring that either the trait or the target type must be defined within the current crate when writing an `impl Trait for Type` block. Prevents conflicting or ambiguous implementations from disparate third-party dependencies.

**Blanket Implementation**:
An implementation of a trait across any type that satisfies given trait bounds (e.g., `impl<T: Display> ToString for T`), automatically equipping vast families of types with standard capabilities.

**`impl Trait` Syntax**:
Syntactic sugar in argument position for an anonymous generic parameter with a trait bound (`item: &impl Summary`). In return position (`-> impl Summary`), specifies that the function returns a single concrete type implementing the trait without exposing its internal concrete name.

**`where` Clause**:
An alternative syntax specifying trait bounds after the function signature and return type, improving readability when multiple generic parameters carry complex or intersecting trait bounds.

**Borrow Checker**:
The compile-time analysis subsystem within `rustc` that tracks reference lifetimes and mutation permissions to guarantee memory safety and eliminate dangling pointers and data races.

**Lifetime Annotation (`'a`)**:
A generic parameter describing the duration for which references are guaranteed to be valid, establishing compile-time relationships across references without altering their runtime duration.

**Lifetime Elision Rules**:
A set of three deterministic compiler heuristics that automatically infer lifetime parameters on function and method signatures, eliminating redundant manual annotations in common patterns.

**`'static` Lifetime**:
A reserved lifetime denoting that data lives for the entire program execution duration (e.g. string literals in `.rodata`). As a trait bound (`T: 'static`), indicates that a type contains no non-static borrowed references.

**Pinning (`Pin<&mut T>`)**:
A wrapper that guarantees the pointee will not be moved in memory until dropped, enabling safe self-referential generator state machines generated by `async`/`await`.

---

## Distributed Systems, Git & Edge Infrastructure

**Write-Ahead Log (WAL)**:
An append-only disk log where mutations are recorded before being applied to the in-memory state or database. Guarantees linearizability and crash recovery.

**Compare-and-Swap (CAS) Lease / Epoch Fencing**:
A distributed coordination primitive where a node claims or renews ownership of a resource (e.g. an addressable actor or cell) using an atomic conditional write in object storage without requiring a consensus cluster.

**LTX (Litestream Transaction Log)**:
A binary transaction log format that encodes page-level diffs of SQLite databases, allowing continuous point-in-time replication to object storage.

**FUSE (Filesystem in Userspace)**:
An operating system interface that allows non-privileged programs to implement virtual filesystems in user space (e.g. `fuser` in Rust) by intercepting POSIX system calls like `open()`, `read()`, and `stat()`.

**Smart HTTP / `pkt-line`**:
The Git wire transport protocol. Data is streamed in packets prefixed with a 4-hex-character length (`pkt-line`), used by `git-upload-pack` (clones/fetches) and `git-receive-pack` (pushes).

**Zero-Copy Slicing**:
Processing network packets, file chunks, or JSON ASTs by referencing sub-slices of an existing memory buffer (`&[u8]`, `bytes::Bytes`, `memmap2`) without copying bytes into intermediate heap allocations.
