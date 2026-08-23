# Chapter 4.1: Ownership, Memory Architecture, and Safety Semantics

Ownership is Rust's central discipline for enforcing memory safety guarantees at compile time without a garbage collector or manual pointer deallocation.

---

## 1. The Core Ownership Rules

Rust enforces three invariants for all heap and stack resources:

1. **Each value in Rust has an owner** (a variable binding).
2. **There can only be one owner at a time** (exclusive, single ownership).
3. **When the owner goes out of scope, the value is dropped** (automatic deallocation via RAII).

---

## 2. Compile-Time Verification vs. Dynamic Runtime Checks

In interpreted languages (Python, JavaScript, Ruby), variable lookup occurs dynamically:
```python
# Python
read(x)  # Raises NameError: name 'x' is not defined
x = True
```
Dynamic variable lookup incurs overhead: every read must verify variable existence and type metadata at runtime.

Rust operates on a zero-cost abstraction philosophy:
```rust
fn main() {
    // read(x); // ERROR[E0425]: cannot find value `x` in this scope
    let x = true;
    read(x);
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}
```
Rust's compiler uses static analysis (definite assignment and lexical scope analysis) to guarantee every variable is initialized before use. At runtime, the binary contains direct register or stack offsets with zero definition-check overhead.

---

## 3. Physical & Virtual Memory Architecture

Understanding ownership requires understanding how the CPU and operating system manage memory.

### 3.1 The Memory Hierarchy & Register Machine
Modern x86-64 processors execute machine instructions using registers and cache hierarchies:
- **Registers** (`%rax`, `%rbx`, `%rsp`, `%rbp`, etc.): Sub-nanosecond access directly inside CPU execution units.
- **L1/L2/L3 Caches**: High-speed SRAM caching hot memory lines (64 bytes per cache line).
- **Main Memory (RAM)**: DRAM accessed via memory controllers with 50 to 100 ns latency.

```
+-------------------------------------------------------------+
| CPU Core                                                    |
|  [ Registers: %rax, %rsp, %rip ]                            |
|  [ L1 Data Cache (32-64 KB, ~1 ns) ]                        |
|  [ L2 Cache (512 KB - 1 MB, ~3-4 ns) ]                      |
+-------------------------------------------------------------+
| L3 Shared Cache (16-64 MB, ~10-20 ns)                       |
+-------------------------------------------------------------+
                              |
+-------------------------------------------------------------+
| Main Memory / RAM (DRAM, ~60-100 ns)                        |
+-------------------------------------------------------------+
```

### 3.2 Virtual Memory, Page Tables, and the MMU
Programs do not execute in raw physical RAM. The Operating System and CPU hardware establish a **Virtual Address Space** for each process.

```
Process Virtual Memory (64-bit space)
0x0000_0000_0000_0000  +---------------------------+  (Null page - Trap / No Access)
                       | Restricted / Unmapped     |
0x0000_0000_0040_0000  +---------------------------+
                       | .text (Executable Code)   |  (Read + Execute)
                       +---------------------------+
                       | .rodata (Constants/Strings)| (Read Only)
                       +---------------------------+
                       | .data / .bss (Globals)    |  (Read + Write)
                       +---------------------------+
                       | Heap (Grows Upwards)      |  (Read + Write, managed by Allocator)
                       |         |                 |
                       |         v                 |
                       |                           |
                       |         ^                 |
                       |         |                 |
                       | Stack (Grows Downwards)   |  (Read + Write, managed by %rsp)
0x0000_7FFF_FFFF_FFFF  +---------------------------+
                       | Kernel Space (Restricted) |  (Ring 0 only)
0xFFFF_FFFF_FFFF_FFFF  +---------------------------+
```

1. **Memory Management Unit (MMU)**: Hardware unit on the CPU that translates virtual addresses to physical RAM frames using multi-level Page Tables (PML4 -> PDPT -> PD -> PT on x86-64).
2. **Translation Lookaside Buffer (TLB)**: High-speed hardware cache of recent virtual-to-physical address translations.
3. **Page Permissions**: Each 4 KB page entry contains protection bits:
   - `R`: Readable
   - `W`: Writable
   - `X` / `NX`: Executable / No-Execute bit

---

## 4. Stack vs. Heap Allocation Mechanics

| Property | Stack | Heap |
| :--- | :--- | :--- |
| **Organization** | LIFO (Last-In, First-Out) stack frames | Unstructured pool of arbitrary sized blocks |
| **Allocation Mechanism** | Single CPU instruction: subtract from `%rsp` (Stack Pointer) | Complex allocator algorithm (`malloc`, `jemalloc`, `mimalloc`) |
| **Deallocation** | Single CPU instruction: add to `%rsp` upon function return | Allocator bookkeeping, freelist updates, coalescing adjacent blocks |
| **Speed** | Instantaneous (~0.5 ns) | Slower (~20 to 100 ns depending on allocator locking & fragmentation) |
| **Size Requirements** | Must have known, fixed size at compile time (`Sized`) | Can be dynamically sized, resized, or unknown at compile time |
| **Data Locality** | Extremely high (keeps L1/L2 CPU cache lines hot) | Lower (pointers scattered across address space cause cache misses) |

### 4.1 The Stack Frame Lifecycle
When a function is called:
1. The `call` instruction pushes the Return Address (`%rip`) onto the stack.
2. The function prologue adjusts `%rsp` downward to allocate space for local variables.
3. When the function returns, the epilogue resets `%rsp` and `ret` pops the instruction pointer.

### 4.2 The Heap and Allocators
When heap memory is requested (e.g. `Box::new`, `String`, `Vec`):
1. The global allocator searches its metadata structures (free-lists, arenas, size-classes) for a suitable contiguous byte range.
2. If available, it marks the segment as used and returns a 64-bit virtual memory pointer (`*mut u8`).
3. If memory is exhausted, the allocator requests additional pages from the OS kernel via the `brk`/`sbrk` or `mmap` system calls.

---

## 5. Hardware Exceptions, Segfaults, and Guard Pages

### 5.1 What is a Segmentation Fault?
A **Segmentation Fault (`SIGSEGV`)** is an operating system signal sent to a process when hardware memory protection is violated.

#### The Step-by-Step Hardware and Kernel Sequence:
```
1. CPU executes instruction referencing virtual address (e.g., mov (%rax), %rbx where %rax = 0x0).
                                |
2. MMU looks up address in TLB / Page Tables.
                                |
3. Page is either:
   - Not mapped in page tables (Invalid Address / Null Pointer)
   - Lacks permissions (e.g. attempting to write to read-only .rodata or code page)
                                |
4. CPU raises hardware exception: Page Fault (#PF, Interrupt 14) or General Protection Fault (#GP, Interrupt 13).
                                |
5. CPU switches to Ring 0 (Kernel Mode) and jumps to OS interrupt descriptor table (IDT).
                                |
6. Linux kernel looks up fault address in process `vm_area_struct` list:
   - Address is invalid -> Kernel sends signal SIGSEGV (signal 11) to the thread.
                                |
7. If no custom signal handler caught it, the OS default handler aborts the process and writes a core dump.
```

### 5.2 Stack Overflow Mechanics
- Every thread is allocated a fixed-size stack (typically 2 MB on Linux threads, 8 MB on main thread).
- At the bottom of the stack region, the OS leaves an unmapped, protected page called a **Stack Guard Page**.
- When nested calls or oversized stack allocations push `%rsp` past the allowed boundary into the guard page, the CPU triggers a `#PF` on the guard page.
- The kernel recognizes the fault at the guard boundary and instantly terminates the process with a stack overflow abort rather than allowing stack corruption of neighboring memory.

```
Higher Addresses  +------------------------+
                  | Frame: main()          |
                  +------------------------+
                  | Frame: helper()        |
                  +------------------------+
                  | %rsp -> current frame  |
                  |          |             |
                  |          v (Grows down)|
                  +------------------------+
                  | [ GUARD PAGE: NO ACCESS]| -> Accessing this triggers immediate SIGSEGV
Lower Addresses   +------------------------+
```

---

## 6. The Memory Safety Spectrum & Common C/C++ Vulnerabilities

Memory safety bugs account for approximately 70% of all high-severity vulnerabilities (CVEs) in systems software according to Microsoft, Google Chromium, and the NSA/CISA reports.

Rust eliminates these vulnerabilities at compile time:

### 6.1 Use-After-Free (UAF)
- **Vulnerability**: Reading or writing through a pointer after the target memory has been deallocated. An attacker can reallocate that memory with malicious payload.
- **Rust Protection**: Ownership system invalidates variables once moved; Borrow checker guarantees references cannot outlive their target.

### 6.2 Double Free
- **Vulnerability**: Calling `free()` twice on the same memory pointer. This corrupts allocator metadata (freelist pointers), allowing arbitrary code execution.
- **Rust Protection**: Exactly one owner exists. The compiler places exactly one `drop()` call when the owner exits scope. Moves statically invalidate previous bindings.

### 6.3 Null Pointer Dereference
- **Vulnerability**: Dereferencing address `0x0` causes immediate crash (`SIGSEGV`) or denial-of-service.
- **Rust Protection**: Rust has no `NULL` pointers in safe code. Missing values must be expressed via the [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html) enum. Null-pointer optimization ensures `Option<Box<T>>` has the exact same 8-byte size as a raw pointer.

### 6.4 Buffer Overflow / Out-of-Bounds Write
- **Vulnerability**: Writing data beyond allocated boundary, overwriting neighboring stack frames, return addresses (`%rip`), or allocator chunks.
- **Rust Protection**: Array and slice accesses (`slice[index]`) perform automatic bounds checking at runtime (or compiler proves bounds at compile time and eliminates checks). Slices carry a pointer and length tuple (Fat Pointer).

### 6.5 Uninitialized Memory Access
- **Vulnerability**: Reading uninitialized stack/heap variables containing sensitive remnants of previous computations (keys, passwords, pointers).
- **Rust Protection**: Definite Assignment Analysis forces every variable to be initialized prior to read access.

### 6.6 Data Races
- **Vulnerability**: Two concurrent threads accessing the same memory location simultaneously where at least one access is a write, without synchronization.
- **Rust Protection**: Aliasing XOR Mutability invariant (`&` shared immutable OR `&mut` exclusive mutable, never both) combined with `Send` and `Sync` traits.

---

## 7. Deep Dive: Memory Layout of `Box<T>` and `String`

### 7.1 Anatomy of `String`
A `String` is a growable, heap-allocated buffer of UTF-8 bytes. On the stack, it occupies 24 bytes (3 words on 64-bit architecture):
- `ptr`: 64-bit pointer to the heap buffer address.
- `len`: 64-bit integer representing current valid byte count.
- `capacity`: 64-bit integer representing total allocated buffer capacity.

```
Stack (`let s = String::from("hello")`)     Heap
+-----------------------------------+        +---+---+---+---+---+
| ptr      | 0x0000_7fff_5000_1000  | -----> | h | e | l | l | o |
| len      | 5                      |        +---+---+---+---+---+
| capacity | 5                      |        (5 bytes allocated)
+-----------------------------------+
```

### 7.2 Anatomy of `Box<T>`
[`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) is an owned smart pointer to heap memory.
- Stack representation: 8 bytes (single pointer on 64-bit target).
- Heap representation: the value `T`.

```rust
let a = Box::new([0; 1_000_000]);
```
- Stack (`a`): 8-byte pointer holding heap address `0x7fff_6000_0000`.
- Heap: 4,000,000 bytes (4 MB) of contiguous zeroed integers (`[i32; 1_000_000]`).

---

## 8. Moves vs Copies vs Clones

### 8.1 Move Semantics (Shallow Copy + Invalidation)

> [!IMPORTANT]
> **Move Semantics vs. Copy Semantics Scope**:
> Invalidation on assignment (`let b = a;`) occurs **only** for heap-allocated data types, dynamic collections, and resource handles that do **not** implement the `Copy` trait. For stack-only types implementing `Copy`, assignment duplicates the stack bytes and leaves the source variable completely valid.

When assigning an owned type (`let b = a;`):

```rust
let a = Box::new([0; 1_000_000]);
let b = a; // Ownership moved to b
// println!("{:?}", a); // COMPILE ERROR: use of moved value `a`
```

1. **Stack Bitwise Copy**: Rust copies the 8-byte pointer from `a` to `b` on the stack.
2. **Invalidation**: The compiler marks `a` as logically uninitialized.
3. **No Heap Mutation**: The 4 MB heap array is not copied or reallocated.
4. **Safety**: When `b` goes out of scope, the heap memory is freed once. Because `a` was invalidated, no double-free can occur.

```
Stack                                Heap
+-----------------------+
| a: [INVALID / MOVED]  |
+-----------------------+
| b: ptr (0x7fff6000)   | ---------> [0, 0, 0, ... 0] (4 MB buffer)
+-----------------------+
```

### 8.2 Type Categorization: Move Types vs. Copy Types

#### 1. Move Types (Heap-Allocated / Resource Owners -> Source Invalidated)
Types that manage dynamic memory on the heap or OS resource handles. Assigning these transfers exclusive ownership and invalidates the previous variable:
- **`String`**: Heap-allocated UTF-8 string buffer.
- **`Vec<T>`**: Heap-allocated growable array.
- **`Box<T>`**: Heap-allocated smart pointer.
- **`HashMap<K, V>` / `BTreeMap<K, V>`**: Heap-allocated associative tables.
- **`File` / `TcpStream`**: Operating system file descriptors and network sockets.
- **Custom `struct` / `enum`**: Default to Move semantics unless all fields implement `Copy` and `#[derive(Copy, Clone)]` is explicitly added.

#### 2. Copy Types (Stack-Only Primitives -> Source Remains Valid)
Types whose entire data resides inline on the stack with no external resources. Assigning these performs an implicit stack duplication:
- **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.
- **Floating-point**: `f32`, `f64`.
- **Boolean**: `bool` (`true`, `false`).
- **Character**: `char` (4-byte Unicode scalar).
- **Fixed-size Arrays**: `[T; N]` where `T: Copy` (e.g. `[i32; 1000]`).
- **Tuples**: `(T1, T2, ...)` where every contained type implements `Copy` (e.g. `(i32, bool)`).
- **Shared References**: `&T` (the reference itself is a Copy pointer).

**Invariant**: Any type that manages external heap resources or implements [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) **cannot** implement `Copy`.

### 8.3 The `Clone` Trait (Explicit Deep Copy)
When duplicate heap memory is required:

```rust
let a = Box::new([0; 1_000_000]);
let b = a.clone(); // Explicit deep allocation
```

- Allocates a new 4 MB buffer on the heap.
- Copies all 4,000,000 bytes from `a`'s heap buffer to `b`'s heap buffer.
- `a` and `b` both remain valid independent owners of separate heap blocks.

```
Stack                     Heap
+-------------------+      +-------------------------------+
| a: ptr (0x1000)   | ---> | Buffer 1 (4 MB) [0, 0, ... 0] |
+-------------------+      +-------------------------------+
+-------------------+      +-------------------------------+
| b: ptr (0x5000)   | ---> | Buffer 2 (4 MB) [0, 0, ... 0] |
+-------------------+      +-------------------------------+
```

### 8.4 Dereferencing (`*a`) onto the Stack
What happens if you run:
```rust
let a = Box::new([0; 1_000_000]);
let b = *a;
```
- `*a` dereferences the `Box` to retrieve the underlying `[i32; 1_000_000]`.
- Because `[i32; 1_000_000]` implements `Copy`, this copies all 4 MB from the heap directly onto `b`'s **stack frame**.
- `b` becomes a stack-allocated 4 MB array.

---

## 9. Functions, Parameters, and Return Values

Passing an argument to a function follows identical semantics to variable assignment (`let`):
- Passing an owned type without `Copy` **moves** ownership into the function's parameter. The caller loses ownership.
- Passing a `Copy` type copies the value onto the callee's stack frame.
- Returning a value transfers ownership from the function's local frame to the caller.

```rust
fn takes_ownership(s: String) {
    println!("{s}");
} // `s` is dropped and freed here

fn makes_copy(x: i32) {
    println!("{x}");
} // `x` goes out of scope, nothing happens

fn gives_ownership() -> String {
    let s = String::from("acquired");
    s // Ownership moved out to caller
}

fn main() {
    let s1 = gives_ownership(); // s1 receives ownership
    takes_ownership(s1);        // s1 moved into function; cannot use s1 below
    
    let x = 42;
    makes_copy(x);              // x is copied; x remains valid below
    println!("{x}");            // Valid
}
```

---

## 10. Summary Matrix: Copy vs Move vs Reference vs Clone

| Operation | Syntax | Stack Work | Heap Work | Source Status | Target Type |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Move** | `let b = a;` | Bitwise copy of descriptor (8/24 B) | None (reused) | **Invalidated** | `Box<T>` / `String` |
| **Copy** | `let b = x;` | Bitwise copy of value (4/8 B) | None | Valid | `i32`, `bool`, `[T; N]` |
| **Clone** | `let b = a.clone();` | Initialize descriptor (8/24 B) | **Allocates new buffer & copies bytes** | Valid | `Box<T>` / `String` |
| **Shared Ref** | `let b = &a;` | Pointer to stack slot / target (8 B) | None | Valid (Immutable) | `&Box<T>` / `&String` |
| **Slice Ref** | `let b = &a[..];` | Fat pointer (ptr + len, 16 B) | None | Valid (Immutable) | `&[T]` / `&str` |
| **Deref to Stack** | `let b = *a;` | Copies inner `T` to stack frame | None | Valid (if `T: Copy`) | `[i32; 1_000_000]` |

---

## 11. Mental Models, Edge Cases & Case Studies

### 11.1 Invalid Code vs. `unsafe` vs. Undefined Behavior (UB)

| Category | Occurs At | Definition | Example |
| :--- | :--- | :--- | :--- |
| **Invalid Code** | Compile Time | Rejected by the compiler's type checker / borrow checker. Zero machine code emitted. | Using a moved `Box` (`[E0382]`) |
| **`unsafe` Block** | Source Code | Programmer manually takes responsibility for invariants where the compiler cannot prove safety. | Calling C FFI or dereferencing raw pointers |
| **Undefined Behavior (UB)** | Runtime | Executing machine code that violates language/compiler contracts. LLVM optimizer makes invalid assumptions. | Reading a dangling pointer after `free()` in C |

### 11.2 The "Drop Follows the Move" Model (Case Study: `move_a_box`)

Consider the thought experiment: *If Rust allowed using moved variables, when would Undefined Behavior occur?*

When a value is moved (`let b2 = b;`), the **drop responsibility transfers exclusively to `b2`**. The moved-from variable `b` is never dropped upon scope exit.

```rust
// Case 1: NO Undefined Behavior (Safe in this specific sequence)
let b = Box::new(0); // 1. Heap allocated at 0x1000
let b2 = b;          // 2. Drop responsibility moved to b2
println!("{}", b);   // 3. Reads 0x1000 (valid! b2 has not been dropped yet)
move_a_box(b2);      // 4. b2 drops and frees 0x1000 once
// End of scope: b does not drop. Exactly 1 free, 0 reads to freed memory.

// Case 2: USE-AFTER-FREE (UB)
let b = Box::new(0);
move_a_box(b);       // 1. Memory at 0x1000 is FREED HERE
println!("{}", b);   // 2. USE-AFTER-FREE! Reads deallocated heap memory.

// Case 3: DOUBLE-FREE (UB)
let b = Box::new(0);
let b2 = b;
move_a_box(b);       // 1. Frees 0x1000
// End of scope: b2 drops and attempts to free 0x1000 AGAIN -> DOUBLE FREE!
```

> [!NOTE]
> Rust rejects all of these snippets unconditionally at compile time to eliminate temporal fragility (swapping lines 3 and 4 in Case 1 instantly creates Case 2).

