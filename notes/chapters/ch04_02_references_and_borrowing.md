# Chapter 4.2: References, Borrowing, and the Pointer Safety Principle

In Chapter 4.1, we explored how ownership guarantees deterministic memory deallocation without a garbage collector. However, passing ownership into every function call requires returning tuples to get the original data back. 

**References and Borrowing** provide a zero-cost mechanism to access and manipulate data without taking ownership.

---

## 1. The Core Hazard: Aliasing Combined with Mutation

Pointers are a fundamental systems programming primitive because they enable **aliasing**: accessing the same memory location through multiple different variable bindings or paths.

On its own, aliasing is harmless (multiple variables reading identical bytes). On its own, mutation is harmless (a single variable modifying its private bytes). 

However, **aliasing combined with mutation is a recipe for disaster**. One variable can "pull the rug out" from another variable in three fatal ways:

```
+---------------------------------------------------------------------------------------------------+
| 1. DEALLOCATION (Dangling Pointers / Use-After-Free)                                              |
|    Variable A deallocates the shared memory buffer. Variable B still points to that address,      |
|    leaving it as a dangling pointer into freed memory.                                            |
+---------------------------------------------------------------------------------------------------+
| 2. IN-PLACE MUTATION & REALLOCATION (Pointer / Iterator Invalidation)                             |
|    Variable A mutates or resizes the data structure (e.g. vector growth), reallocating the buffer |
|    to a new heap address. Variable B is left pointing to the stale, deallocated address.           |
+---------------------------------------------------------------------------------------------------+
| 3. CONCURRENT MUTATION (Data Races)                                                               |
|    Thread A writes to memory while Thread B reads or writes without synchronization, causing      |
|    nondeterministic CPU cache tearing and memory corruption.                                      |
+---------------------------------------------------------------------------------------------------+
```

### The Pointer Safety Principle
> **Data must never be simultaneously aliased AND mutable.**
> 
> Rust enforces this invariant statically via the borrow checker: at any point in time, memory may have **any number of read-only aliases (`&T`)** OR **exactly one mutable access path (`&mut T`)**, but never both.

---

## 2. Shared References (`&T` / Immutable Borrows)

A reference is a non-owning pointer (8 bytes on 64-bit architectures) pointing to an existing memory address.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it does not own the String, nothing is dropped.

fn main() {
    let s1 = String::from("systems");
    let len = calculate_length(&s1); // Passes a reference; s1 retains ownership
    println!("The length of '{s1}' is {len}."); // Valid: s1 was not moved
}
```

### Stack and Heap Memory Representation

```
Stack (`main`)                             Heap
+-----------------------------------+      +---+---+---+---+---+---+---+
| s1: ptr      (0x7fff_5000_1000)   | ---> | s | y | s | t | e | m | s |
|     len      (7)                  |      +---+---+---+---+---+---+---+
|     capacity (7)                  |
+-----------------------------------+
                  ^
Stack (`calculate_length`) |
+------------------------+ |
| s: ptr (0x7ffe_0010)   |-+ (Points to s1 on main's stack frame)
+------------------------+
```

---

## 3. Mutable References (`&mut T` / Exclusive Borrows)

Mutable references allow modifying borrowed data without taking ownership.

```rust
fn append_suffix(s: &mut String) {
    s.push_str("-core");
}

fn main() {
    let mut s = String::from("engine");
    append_suffix(&mut s);
    println!("{s}"); // Prints: engine-core
}
```

### The Single Mutable Reference Invariant
You can have only **one** mutable reference to a piece of data in a particular scope:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // ERROR[E0499]: cannot borrow `s` as mutable more than once at a time

println!("{r1}, {r2}");
```

This restriction prevents data races at compile time. A **data race** occurs when:
1. Two or more pointers access the same memory concurrently.
2. At least one pointer is writing.
3. No synchronization mechanisms (mutexes, atomics) are used.

---

## 4. The Borrow Checker Permission Model (The R-W-O Flow)

The borrow checker evaluates memory safety by tracking three permissions for every path (variable, pointer dereference, field access):

- **`R` (Read)**: Permission to read or copy data from this place.
- **`W` (Write)**: Permission to mutate or overwrite data at this place in-place.
- **`O` (Own)**: Permission to move, transfer, or drop this place.

### 4.1 Differentiating Variables vs Referents (`num` vs `*num`)

When you create a reference:
```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
```

Two distinct places are created in the permission tracker:

| Place | Description | Read (`R`) | Write (`W`) | Own (`O`) | Architectural Reason |
| :--- | :--- | :---: | :---: | :---: | :--- |
| **`num`** | Pointer on stack | **+R** | **-** | **+O** | You own the 8-byte pointer variable on the stack. You can move or drop it. |
| **`*num`** | Element in heap | **+R** | **-** | **-** | You only borrowed the integer in heap RAM. **No Ownership (`-O`)**. You cannot free/move it. |

---

### 4.2 Why Root Owners (`v`) Lose Read Permissions During `&mut` Borrows

Consider a mutable borrow:
```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];
// At this point: v has lost ALL permissions (R, W, O are all [-, -, -])
```

#### Why can't `v` even be read while `num` is active?
1. **Total Isolation for Writers**: An `&mut` reference guarantees exclusive access. The writer knows no other code is observing memory while it is potentially in an inconsistent, mid-mutation state.
2. **Preventing Data Races**: If `v` retained `R` while `num` has `W`, multithreaded readers could read torn memory while `num` writes.
3. **Restoration via NLL**: The moment `num` is used for the last time, the borrow ends and all permissions (`+R, +W, +O`) are immediately restored back to `v`.

---

## 5. Reborrowing & Permission Hierarchies (`v -> num -> num2`)

When you create a reference from another reference (reborrowing), permissions form a **hierarchical tree**:

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];
let num2: &i32 = &*num; // Reborrows immutably from *num
```

```
Borrow Tree:
v (Root Owner: LOCKED [-, -, -])
 └── num (&mut v[2]: Reborrowed, temporarily lost W: [R, \W, \O])
      └── num2 (&*num: Active Reader: [+R, -, +O])
```

- **`num2` borrows from `num`, NOT from `v`**.
- `*num` temporarily loses `W` so it cannot write while `num2` reads.
- **`v` DOES NOT regain `R` permission!** The root owner `v` remains completely locked with zero permissions until the entire borrow tree (`num` and `num2`) has finished executing.

---

## 6. Collection-Level Locking vs. Struct Field Splitting

Why does `&mut v[2]` lock the **entire vector `v`** instead of just index 2?

### 6.1 Indexing is a Method Call (`IndexMut`)
`&mut v[2]` is syntactic sugar for `v.index_mut(2)`. The method signature takes `&mut self` (the whole vector).

### 6.2 Dynamic Runtime Indices
In real-world code, indices are often dynamic runtime values (`v[i]`, `v[j]`). Because the compiler cannot prove statically whether `i == j`, it conservatively locks the **entire collection** to prevent accidental aliased mutation.

### 6.3 Contrast: Struct Fields DO Support Disjoint Borrowing
Struct fields have fixed, static memory offsets known at compile time:

```rust
struct Point { x: i32, y: i32 }
let mut p = Point { x: 10, y: 20 };

// ALLOWED: The compiler proves statically that p.x and p.y are distinct addresses
let rx = &mut p.x;
let ry = &mut p.y;
*rx += 1;
*ry += 2;
```

### 6.4 Mutably Borrowing Multiple Vector Elements Safely
To borrow multiple vector elements mutably without locking issues, use standard library primitives that prove disjoint memory ranges:
- **`v.split_at_mut(mid)`**: Splits a slice into two non-overlapping mutable slices (`&mut [T]`).
- **`v.iter_mut()`**: Iterates over elements providing safe, sequential `&mut T` handles.

---

## 7. Physical Architecture Deep Dive: Vector Reallocation & Pointer Invalidation

Why does modifying a vector (`v2.push(4)`) while an alias exists cause a **Use-After-Free (label L1)**?

```
[ Step 1: Initial Allocation ]
let v1 = vec![1, 2, 3]; // Capacity = 3, Buffer at 0x1000
let mut v2 = v1;        // Move: v1 invalidated, v2 owns 0x1000

Stack                                      Heap (Address: 0x1000, Capacity: 3)
v1: [MOVED / INVALID]
v2: ptr = 0x1000, len=3, cap=3     --->    [ 1, 2, 3 ]


[ Step 2: v2.push(4) Triggers Reallocation ]
Because capacity is full:
1. Allocator grants a new buffer at address 0x5000 (capacity 6).
2. Elements [1, 2, 3] are copied to 0x5000 and 4 is appended.
3. Allocator CALLS free(0x1000) TO DEALLOCATE THE OLD HEAP BUFFER.

Stack                                      Heap
v1: (statically holds stale ptr 0x1000) -> [OLD HEAP BUFFER 0x1000 FREED / DEALLOCATED]
v2: ptr = 0x5000, len=4, cap=6     --->    [ 1, 2, 3, 4 ] (at 0x5000)


[ Step 3: Attempting to Read v1[0] ]
If permitted, reading v1[0] dereferences stale address 0x1000.
This is a USE-AFTER-FREE (L1): reading memory after its pointee has been freed.
Rust eliminates this at compile time by marking v1 invalid upon move.
```

---

## 8. Non-Lexical Lifetimes (NLL)

Modern Rust uses **Non-Lexical Lifetimes (NLL)**: a reference's lifetime ends at the **last point it is actually used**, rather than the closing curly brace `}` of the block.

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    let num = &v[2]; // Shared borrow starts (v loses W and O)
    println!("Third element is {}", *num); // LAST USE OF `num`!
    
    // NLL: `num` is dead. Permissions are restored to `v` (+W, +O) immediately!

    v.push(4); // COMPILES! `v` has Write permission restored.
    println!("Vector is now: {:?}", v);
}
```

---

## 9. Dangling References: Prevention at Compile Time

Returning a pointer to a stack-allocated local variable causes immediate memory corruption in C/C++:

```c
// C - Undefined Behavior
int* dangle() {
    int x = 42;
    return &x; // Returns pointer to stack frame about to be popped!
}
```

Rust rejects this statically:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s // ERROR[E0106]: missing lifetime specifier / returns reference to local variable
} // `s` dropped here. Returning `&s` would create an instant dangling pointer.
```

**The Fix**: Transfer ownership across stack frame boundaries by returning the value directly:
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership moved to caller
}
```

---

## 10. Summary Matrix: The Golden Rules of Borrowing

1. **Aliasing XOR Mutability**: At any time, you may have any number of `&T` references OR exactly one `&mut T` reference, but never both.
2. **References Must Always Be Valid**: No reference can ever outlive its referent (no dangling references, no use-after-free).
3. **Hierarchical Lock Trees**: Borrowing from a reference locks both the reference and the root owner until the entire sub-tree of borrows completes.
4. **NLL Permission Restoration**: Permissions taken by a borrow are returned to the owner immediately after the borrow's final usage line.
