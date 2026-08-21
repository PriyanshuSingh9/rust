# 🦀 Array Slicing, Unsized Types, and Owned Copies

Why `let b = a[1..4];` fails, what slices really are, and how to get independent
copies instead of views. Companion code: `rust_book/basics/practice/src/main.rs`
(`slices_and_copies`).

---

## 1. What a Range Index Produces

```rust
fn check() {
    let a: [i32; 5] = [3; 5];
    let b = a[1..4]; // ERROR: does not compile
}
```

`a[1..4]` means "elements at indices 1, 2, 3", but range indexing does **not**
produce another array. It produces a slice: `[i32]`.

A slice is an **unsized (dynamically sized) type**. Rust must know every local
variable's size at compile time to reserve stack space, and a slice's length is
only known at runtime. So `[i32]` cannot be stored as a plain local variable:

```text
error[E0277]: the size for values of type `[i32]` cannot be known at compile time
```

Even though the compiler *could* figure out that indices 1..4 of an `[i32; 5]`
are 3 elements, the length is not tracked in the type. Range slicing is defined
to return `[T]`, never a fixed-size array. Only single indexing (`a[2]`) copies
a value out, because one element always has a known size.

---

## 2. The Three Sizes That Matter

| Type      | Kind                  | Size known? | Storable as local? |
|-----------|-----------------------|-------------|--------------------|
| `[i32; 5]`| array                 | yes         | yes                |
| `[i32]`   | slice                 | no          | no                 |
| `&[i32]`  | slice reference       | yes         | yes                |

A bare slice can only exist **behind a pointer**: `&[T]`, `&mut [T]`,
`Box<[T]>`. The reference is a *fat pointer* carrying two words:

```text
&[i32]
+----------------+----------------+
| data pointer   | length         |
| -> first elem  | number of elems|
+----------------+----------------+
```

That is why the `&` is mandatory: `&v[1..4]` binds a fat pointer, which is a
normal sized value.

---

## 3. Range Syntax Cheat Sheet

```rust
let v = [10, 20, 30, 40, 50];

let full      = &v[..];    // all elements
let mid       = &v[1..4];  // [20, 30, 40] - end is EXCLUSIVE
let first_two = &v[..2];   // [10, 20]
let rest      = &v[2..];   // [30, 40, 50]
```

- Zero-copy: these point into `v`'s buffer, they do not duplicate data.
- Bounds-checked at runtime: `v[0..10]` panics.
- Works identically on arrays and `Vec`s because both coerce to `&[T]`.
  (`Vec<T>` dereferences to `[T]`; arrays `[T; N]` coerce to `[T]`. Range
  slicing is really an operation on slices.)

String slices are the same idea: `"hello"[1..3]` is `"el"` of type `&str`,
with the extra rule that cuts must land on UTF-8 character boundaries.

---

## 4. Independent Copies (Not Views)

`&v[1..4]` borrows: writes through the slice affect the original. When you need
duplicate data that the caller owns outright:

### Copy into an owned Vec

```rust
let owned: Vec<i32> = v[1..4].to_vec();
owned[0] = 99; // v untouched
```

Heap allocates and copies the elements. Requires `T: Clone` (trivially true for
`i32`). This is the idiomatic choice when the range is dynamic.

### Copy into a fixed-size buffer without allocating

```rust
let mut trio = [0i32; 3];
trio.copy_from_slice(&v[1..4]); // trio = [20, 30, 40]
```

Copies *into* an existing buffer; lengths must match exactly or it panics.

### Build a new array by hand

```rust
let b = [a[1], a[2], a[3]]; // [i32; 3], fully independent
```

Only practical when indices are known at compile time.

### Whole-value clones

```rust
let arr_copy = v;          // [i32; N] where T: Copy -> full stack copy
let vec_copy = my_vec.clone(); // deep copy of a Vec
```

---

## 5. Handing Data Out of Functions

Choose the return type based on what the caller needs:

```rust
// Borrowed: zero-copy, but caller cannot outlive the input.
fn borrowed(nums: &[i32]) -> &[i32] {
    &nums[1..4]
}

// Owned: caller gets independent data it can keep and mutate freely.
fn owned(nums: &[i32]) -> Vec<i32> {
    nums.to_vec()
}
```

Decision rule: return a borrow (`&[T]`) by default; return an owned `Vec<T>`
only when the value must outlive the input or be mutated independently.

Also prefer slice parameters over concrete containers:

```rust
fn sum(nums: &[i32]) -> i32 // accepts &[i32; 5], &Vec<i32>, sub-slices...
```

---

## 6. Why Rust Does Not Copy on Slice Automatically

If every range index silently copied elements, then:

```rust
let b = huge_array[100..900];
```

would perform a surprise allocation/copy. Rust makes copying explicit:
indexing borrows by default, and you opt into ownership with `.to_vec()`,
`.clone()`, or `copy_from_slice`. If you find yourself cloning just to satisfy
the borrow checker, that is usually a design smell worth revisiting.

---

## 7. Preferred Copy Approach by Scenario

**Sub-range, size unknown until runtime -> `.to_vec()`**

```rust
fn take_first_n(nums: &[i32], n: usize) -> Vec<i32> {
    nums[..n].to_vec()
}
```

The default choice whenever the caller needs to own data that outlives the
input: returning from functions, storing in structs, sending across threads.

**Destination buffer already exists -> `copy_from_slice`**

```rust
let mut header = [0u8; 16];
header.copy_from_slice(&bytes[0..16]);
```

Parsing binary formats, network protocols, embedded/no-alloc code. Zero
allocation; panics if lengths mismatch, which conveniently catches off-by-ones.

**Whole fixed-size array -> plain assignment**

```rust
let a = [1, 2, 3];
let b = a; // full stack copy; arrays of Copy types copy on assignment
```

No method call needed for `[T; N]` where `T: Copy`.

**Whole `Vec`, struct, or nested data -> `.clone()`**

```rust
let v2 = v.clone();
```

For anything heap-allocated or non-`Copy`. For your own types,
`#[derive(Clone)]` and it copies recursively field by field.

**Expensive-to-copy data shared across owners -> don't copy, share**

```rust
use std::sync::Arc;
let shared = Arc::new(big_data);
```

When multiple owners need a handle but only read access, `Rc`/`Arc` beats
deep-copying megabytes.

**Anti-scenarios (copying is the wrong tool):**

- Just iterating or reading -> take `&[T]`
- Mutating in place -> take `&mut [T]`
- Cloning solely to appease the borrow checker -> restructure lifetimes instead

Rule of thumb: **borrow by default, copy at trust boundaries** - where data
leaves your function or crosses into another owner/thread.

---

## 8. How Other Languages Compare

| Language | Slice of builtin | Slice of buffer type |
|----------|------------------|----------------------|
| Python   | copy (`list[1:4]`, shallow) | view (NumPy arrays) |
| JS       | copy (`arr.slice(1, 4)`)    | view (TypedArray `.subarray()`) |
| C++      | view (`std::span`)          | view |
| Rust     | view (`&[T]`)               | view |

Python and JS default to the convenient operation (copy); Rust and C++ default
to the cheap one (view) and make copying explicit. NumPy, TypedArrays, and
`std::span` all converge on the Rust model for performance. Note that C++
`std::span` is a slice without the safety: unchecked bounds and no lifetime
enforcement.

---

## 9. Summary Table

| Expression            | Type      | Semantics                          |
|-----------------------|-----------|------------------------------------|
| `&v[1..4]`            | `&[T]`    | borrowed view into original        |
| `&mut v[1..4]`        | `&mut [T]`| borrowed view, mutable             |
| `v[1..4].to_vec()`    | `Vec<T>`  | owned heap copy                    |
| `buf.copy_from_slice(&v[1..4])` | `()` | copy into existing sized buffer |
| `[v[1], v[2], v[3]]`  | `[T; 3]`  | owned fixed-size array             |
| `v.clone()` / `let w = v;` | same as `v` | full independent duplicate   |

Mental model: almost everything in Rust is an expression with a type. `loop`
evaluates to its `break` value (default `()`); range indexing evaluates to a
slice `[T]`, which is unsized; references to unsized types (`&[T]`) are sized
fat pointers and are what you actually store and pass around.
