# Chapter 8: Common Collections (Vectors, Strings, and Hash Maps)

This notebook provides a first-principles architectural analysis of Chapter 8 of the Rust Book, detailing heap layout, reallocation mechanics, pointer invalidation prevention, UTF-8 internal encoding, and the `HashMap` Entry API.

---

## 1. Memory Layout Overview of Standard Collections

Standard collections are stack-allocated descriptors (fat pointers or tri-word structs) that manage dynamically sized, growable heap buffers.

```text
Stack Frames (64-bit Architecture):

Vec<T> (24 Bytes)
+-----------------------+-----------------------+-----------------------+
| ptr: *mut T (8 B)     | cap: usize (8 B)      | len: usize (8 B)      |
+-----------------------+-----------------------+-----------------------+
           |
           v
Heap: [ element_0 | element_1 | element_2 | ... uninitialized capacity ... ]

String (24 Bytes) - Guaranteed Valid UTF-8 Buffer
+-----------------------+-----------------------+-----------------------+
| ptr: *mut u8 (8 B)    | cap: usize (8 B)      | len: usize (8 B)      |
+-----------------------+-----------------------+-----------------------+
           |
           v
Heap: [ byte_0 | byte_1 | byte_2 | ... valid UTF-8 sequence ... ]

&str / &[T] Slices (16 Bytes) - Non-owning Views
+-----------------------+-----------------------+
| ptr: *const u8/T (8B) | len: usize (8 B)      |
+-----------------------+-----------------------+
```

---

## 2. Vectors (`Vec<T>`)

A vector is a growable, contiguous array allocated on the heap. All elements must be of the same type `T`.

### 2.1 Initialization & Growth Strategy
- `Vec::new()`: Zero allocation on the stack until elements are pushed (`cap = 0`, `ptr = dangling aligned non-null`).
- `vec![1, 2, 3]`: Macro initializing elements with exact capacity.
- `Vec::with_capacity(n)`: Pre-allocates space on the heap, avoiding amortized reallocation overhead during successive pushes.

### 2.2 Element Access & Bound Checking

```rust
let v = vec![10, 20, 30, 40, 50];

// 1. Direct Indexing (Panics if index >= len)
let third: &i32 = &v[2];

// 2. Safe Retrieval with Option (Non-panicking)
let third_safe: Option<&i32> = v.get(2);
match third_safe {
    Some(val) => println!("Found element: {val}"),
    None => println!("Index out of bounds"),
}
```

### 2.3 The Reallocation Pointer Invalidation Hazard
In languages without borrow checking (like C/C++), holding a pointer to an element while appending to a vector leads to **Use-After-Free** bugs when the vector exceeds capacity and reallocates its buffer to a new memory address.

Rust prevents this at compile time via `E0502`:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // Immutable borrow of `v`

v.push(6); // Mutable borrow of `v` (May reallocate entire heap buffer!)

// ERROR[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
// println!("First element: {first}");
```

```text
Pointer Invalidation Timeline:
Heap (Old Buffer): [ 1 | 2 | 3 | 4 | 5 ] <--- `first` references this memory address
                         |
                         v `v.push(6)` exceeds capacity -> allocates new buffer & frees old
Heap (Old Buffer): [ FREED MEMORY / DANGLING POINTER ]
Heap (New Buffer): [ 1 | 2 | 3 | 4 | 5 | 6 | _ | _ ]
                         |
                         v Borrow checker rejects reading `first` after `v.push(6)`
```

### 2.4 Iteration Patterns
```rust
let mut v = vec![100, 32, 57];

// Immutable traversal
for item in &v {
    println!("{item}");
}

// Mutable in-place transformation (requires dereferencing `*`)
for item in &mut v {
    *item += 50;
}
```

### 2.5 Heterogeneous Vectors via Enums
When multiple types must be stored in a single contiguous list, wrap them in an enum:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

---

## 3. UTF-8 Strings (`String` vs. `&str`)

Rust strings are collections of bytes (`Vec<u8>`) verified to be valid UTF-8.

### 3.1 Why Integer Indexing (`s[0]`) Is Forbidden
In Rust, `s[0]` is rejected by the compiler for two fundamental reasons:

1. **UTF-8 Byte Length Variance**: Characters in UTF-8 range from 1 to 4 bytes in length:
   - ASCII characters (`A`-`Z`): 1 byte.
   - Cyrillic / Greek (`З`, `д`): 2 bytes.
   - Devanagari / CJK (`न`, `日`): 3 bytes.
   - Emojis / Extended scripts: 4 bytes.
   - `&"Здравствуйте"[0]` would return the first raw byte `208`, which is not a valid character on its own.

2. **$O(1)$ Performance Invariant**: Indexing operations in systems programming imply constant-time access. Because UTF-8 is a variable-length encoding, finding the $N$-th character requires scanning from byte 0 in $O(N)$ time.

### 3.2 The Three Perspectives of String Representation

For the Hindi word **"नमस्ते"**:

```text
1. Bytes (18 bytes):
   [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

2. Unicode Scalar Values / chars (6 elements):
   ['न', 'म', 'स', '्', 'त', 'े']

3. Grapheme Clusters / Human Letters (4 elements):
   ["न", "म", "स्", "ते"]
```

### 3.3 Safe String Traversal
```rust
let word = "नमस्ते";

// Iterate as Unicode scalar values (chars)
for c in word.chars() {
    println!("{c}");
}

// Iterate as raw underlying bytes
for b in word.bytes() {
    println!("{b}");
}
```

### 3.4 String Concatenation and Deref Coercion
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

// The `+` operator calls `fn add(self, s: &str) -> String`
// `s1` is moved (consumed), `&s2` is coerced from `&String` to `&str` via Deref Coercion:
let s3 = s1 + &s2; 

// Non-consuming formatting macro (leaves s1 and s2 intact):
let formatted = format!("{s2} - {s3}");
```

---

## 4. Hash Maps (`HashMap<K, V>`)

A `HashMap<K, V>` stores mappings of keys of type `K` to values of type `V` using a hashing algorithm over a heap-allocated hash table.

### 4.1 Ownership Invariants
- Types that implement `Copy` (`i32`, `bool`) are copied into the hash map.
- Owned types (`String`, `Vec<T>`) are moved, transferring ownership to the map.

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// `field_name` and `field_value` are invalid here due to move semantics!
```

### 4.2 The `Entry` API: Atomic Lookup and Mutation
The `entry` API provides an $O(1)$ pattern to check, insert, or mutate key-value pairs without redundant hash lookups:

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut word_counts = HashMap::new();

for word in text.split_whitespace() {
    // `entry(word)` returns an `Entry` enum (Occupied or Vacant).
    // `or_insert(0)` inserts 0 if Vacant, returning a `&mut V` reference to the value:
    let count: &mut u32 = word_counts.entry(word).or_insert(0);
    *count += 1;
}

// Result: {"hello": 1, "world": 2, "wonderful": 1}
```

### 4.3 Hashing Function & DoS Resistance
- **Default Hasher**: Rust uses **SipHash 1-3** by default. It provides cryptographically strong protection against HashDoS (Hash Denial of Service) attacks where malicious actors craft hash collision payloads to degrade lookups from $O(1)$ to $O(N)$.
- **Custom Fast Hashers**: For CPU-bound internal workloads (e.g. compilers, game engines) where keys are trusted, you can swap `BuildHasher` with faster non-cryptographic algorithms like `FxHash` or `AHash`.
