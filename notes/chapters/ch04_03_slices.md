# Chapter 4.3: The Slice Type, Memory Representation & UTF-8 Invariants

Slices are non-owning references to a contiguous sequence of elements in a collection. Because slices are references, they do not have ownership and introduce zero runtime allocation overhead.

---

## 1. The State Desynchronization Problem (Why Slices Exist)

To understand why Rust introduced slices, consider the problem of finding the first word in a string.

### 1.1 The Vulnerable Approach (Returning Independent Integer Indices)

```rust
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word_end = first_word_index(&s); // Returns usize 5

    s.clear(); // Empties the String, setting len to 0 and deallocating contents

    // DISASTER: `word_end` is still 5!
    // The index 5 is completely disconnected from the actual state of `s`.
    // Attempting to use `word_end` with `s` later causes out-of-bounds panics or logical bugs.
    println!("Word end index: {word_end}");
}
```

**The Flaw**: The integer `word_end` is an independent value on the stack with no lifetime or borrow-tracking connection to `s`. Mutating or clearing `s` leaves `word_end` stale and invalid.

---

## 2. String Slices (`&str`) as the Safe Solution

A **String Slice** (`&str`) is a reference to a sub-part of a `String` (or any UTF-8 byte buffer).

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // Shared borrow (&str) of `s` starts here!

    // s.clear(); // COMPILE ERROR[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("The first word is: {word}");
}
```

### How the Borrow Checker Eliminates the Desynchronization Bug
1. `first_word(&s)` returns `&str`, which holds an **active immutable borrow (`&`)** of `s`.
2. `s.clear()` requires a **mutable borrow (`&mut s`)** to truncate the buffer.
3. The compiler enforces the **Aliasing XOR Mutability** invariant: `s.clear()` is blocked at compile time as long as `word` is alive, guaranteeing `word` never points to stale or cleared memory.

---

## 3. Physical Memory Representation: Thin vs. Fat Pointers

### 3.1 `&String` (Thin Pointer)
- **Stack size**: 8 bytes.
- **Indirection**: 2 steps (Reference on stack $\to$ `String` header on stack $\to$ Heap buffer).

```
Stack                                                        Heap
+-------------------+
| ref: ptr (0x1000) | ----------------+
+-------------------+                 |
                                      v
+-----------------------------------+--------------------+   +---+---+---+---+---+
| s:   ptr (0x5000), len=5, cap=8   | (24-byte String)   |-> | h | e | l | l | o |
+-----------------------------------+--------------------+   +---+---+---+---+---+
```

### 3.2 `&str` (Fat Pointer)
- **Stack size**: 16 bytes (8-byte pointer + 8-byte length).
- **Indirection**: 1 step (Direct pointer to contiguous bytes in Heap, Stack, or `.rodata`).

```
Stack (`let word = &s[0..5];`)                              Heap
+-----------------------------------+
| word: ptr = 0x5000, len = 5       | ---------------------> [ 'h', 'e', 'l', 'l', 'o' ]
+-----------------------------------+
```

---

## 4. Range Syntax and Permutations

Rust provides concise syntax for sub-slice ranges:

```rust
let s = String::from("hello world");

let len = s.len();

let slice1 = &s[0..5]; // Explicit start and end: "hello"
let slice2 = &s[..5];  // Starts from index 0: "hello" (identical to [0..5])
let slice3 = &s[6..len]; // Slices from index 6 to the end: "world"
let slice4 = &s[6..];    // Omits end index: "world" (identical to [6..len])
let slice5 = &s[..];     // Entire string slice: "hello world" (identical to [0..len])
```

---

## 5. UTF-8 Invariants and Character Boundary Panics

> [!CAUTION]
> **Slice indices in Rust specify BYTE offsets, NOT character counts.**

In UTF-8, characters take variable byte lengths (1 to 4 bytes):
- ASCII English (`a`, `Z`, `1`): **1 byte**
- Cyrillic / Greek / Latin accents (`д`, `é`, `Ω`): **2 bytes**
- CJK (Chinese / Japanese / Korean: `中`, `日`): **3 bytes**
- Extended glyphs and emojis: **4 bytes**

```
Memory byte offsets for `let s = "中文";` (6 bytes total):
Byte Index:     0          1          2          3          4          5
Bytes in RAM: [ 0xE4  |   0xB8   |   0xAD  ]  [ 0xE6  |   0x96   |   0x87  ]
Character:    <-------- '中' (3 bytes) ---->  <-------- '文' (3 bytes) ---->
```

### The Boundary Panic Trap
```rust
let s = "中文";

let ok = &s[0..3]; // Valid: extracts all 3 bytes of '中'
// let bad = &s[0..2]; // RUNTIME PANIC! Cuts inside the 3-byte '中' character!
```

If you slice in the middle of a multi-byte sequence, Rust stops execution with a runtime panic:
```text
thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中文`'
```

### Safe Unicode Processing Tools
- **`.chars()`**: Iterates over full `char` (Unicode Scalar Values) ignoring byte widths.
- **`.char_indices()`**: Yields `(byte_offset, char)` tuples identifying verified character start boundaries.

---

## 6. String Literals Are Slices (`&'static str`)

When you write a string literal:
```rust
let s: &'static str = "Hello, world!";
```
- The bytes `"Hello, world!"` are baked directly into the `.rodata` (read-only data) section of the compiled executable binary.
- `s` is a 16-byte fat pointer pointing to `.rodata`.
- It has the `'static` lifetime, meaning it remains valid for the entire duration of the program.

---

## 7. Idiomatic API Design: String Slices as Function Parameters

[`String`](https://doc.rust-lang.org/std/string/struct.String.html) implements [`Deref<Target = str>`](https://doc.rust-lang.org/std/ops/trait.Deref.html). 

By taking `&str` instead of `&String`, a function accepts all of the following with zero runtime conversion cost:
1. `&String` (automatically coerced via Deref Coercion)
2. `&str` slices (e.g. `&s[0..5]`)
3. String literals (`"hello"`)

```rust
// SUPERIOR API: Accepts String, &String, &str, and literals
fn format_greeting(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let owned = String::from("Ferris");
    let literal = "Rustacean";

    format_greeting(&owned);      // Deref coercion from &String to &str
    format_greeting(literal);     // Native &str
    format_greeting(&owned[0..3]);// Sub-slice &str
}
```

---

## 8. Slices of Arrays and Vectors (`&[T]`)

Slicing is not restricted to strings. Any contiguous collection (`[T; N]` or `Vec<T>`) can be sliced into `&[T]` (immutable slice) or `&mut [T]` (mutable slice):

```rust
let arr: [i32; 5] = [10, 20, 30, 40, 50];

// Slicing a fixed-size stack array:
let slice: &[i32] = &arr[1..4]; // Holds elements [20, 30, 40], len = 3

// Slicing a heap-allocated vector:
let mut vec = vec![1, 2, 3, 4, 5];
let (left, right) = vec.split_at_mut(2); // left is &mut [1, 2], right is &mut [3, 4]
```

### Memory Layout of `&[T]`
A slice `&[T]` is always a **16-byte fat pointer** on 64-bit architectures:
- `ptr`: 8-byte pointer to the first element.
- `len`: 8-byte integer representing the element count.

---

## 9. Why `drop(s_ref)` Does Nothing on References

```rust
let s = String::from("Hello");
let s_ref = &s;
drop(s_ref);        // NO-OP! Compiler emits a warning
println!("{s_ref}"); // Compiles and runs cleanly!
```

**Architectural Reasons**:
1. **References Implement `Copy`**: Shared references (`&T`) are 8-byte/16-byte stack descriptors implementing the `Copy` trait. Passing `s_ref` to `drop` makes a bitwise copy of the pointer on `drop`'s stack frame. The original `s_ref` is never moved.
2. **No `Drop` Destructor**: References do not own the underlying memory, so dropping a reference executes zero deallocation code.

---

## 10. Summary Reference Table

| Type | Size on Stack | Indirection Steps | Memory Residency of Data | Ownership |
| :--- | :--- | :--- | :--- | :--- |
| **`String`** | 24 bytes (`ptr`, `len`, `cap`) | 1 | Heap | **Owner** (frees on drop) |
| **`&String`** | 8 bytes (`ptr`) | 2 | Stack header $\to$ Heap | **Borrow** (cannot outlive `String`) |
| **`&str`** | 16 bytes (`ptr`, `len`) | 1 | Heap, Stack, or `.rodata` | **Borrow** (fat pointer) |
| **`&[T]`** | 16 bytes (`ptr`, `len`) | 1 | Heap, Stack, or Static | **Borrow** (fat pointer) |
