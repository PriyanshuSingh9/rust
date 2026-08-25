# Chapter 6: Enums, Pattern Matching, and Algebraic Data Types

This notebook provides a first-principles architectural analysis of Chapter 6 of the Rust Book, covering algebraic sum types, memory layouts, niche optimization, the `Option<T>` enum, exhaustive pattern matching, and concise control flow.

---

## 1. Algebraic Data Types: Product Types vs. Sum Types

Type theory classifies composite data types into **Product Types** and **Sum Types**:

| Concept | Rust Primitive | Mathematical Representation | Memory Allocation Behavior |
| :--- | :--- | :--- | :--- |
| **Product Type** | `struct` / `tuple` | $A \times B$ (Carries $A$ **AND** $B$) | All fields coexist simultaneously on the stack ($\sum \text{size}(field_i)$). |
| **Sum Type** | `enum` | $A + B$ (Carries $A$ **OR** $B$) | Variants share overlapping memory ($\max(\text{size}(variant_i)) + \text{tag}$). |

```rust
// Product Type: Size = size_of(u32) + size_of(u32) = 8 bytes
struct Point {
    x: u32,
    y: u32,
}

// Sum Type: Size = max(size_of(u32), size_of(String)) + tag + padding
enum Payload {
    Numeric(u32),
    Text(String),
}
```

---

## 2. Memory Layout of Enums & Tagged Unions

An `enum` with payloads is compiled as a **tagged union** consisting of:
1. **Discriminant (Tag)**: An integer identifying which variant is currently active.
2. **Payload Region**: A chunk of memory sized to the largest variant.

### Concrete Stack Memory Layout

```text
Stack Frame: Payload::Text(String) (Assuming 64-bit architecture)
+---------------+-----------------------+-----------------------+-----------------------+
| Tag (1 byte)  | Padding (7 bytes)     | ptr: *const u8 (8 B)  | cap: usize (8 B)      | len: usize (8 B)      |
+---------------+-----------------------+-----------------------+-----------------------+
|<--- Discriminant Block (8 B) -------->|<---------------- Payload Union Block (24 B) ------------------->|
Total Size = 32 bytes (aligned to 8 bytes)
```

If the variant is `Payload::Numeric(42)`, the same 24-byte payload region stores the 4-byte `u32` (with trailing unused capacity).

---

## 3. Niche Optimization (Null-Pointer Optimization)

When an enum wraps a type that contains **invalid bit patterns (niches)**, the compiler eliminates the discriminant tag entirely, resulting in zero memory overhead.

### Example: `Option<&T>` and `Option<NonNull<T>>`
A standard Rust reference `&T` can never be null (`0x0`). The compiler utilizes this niche:
- `Some(&T)` $\rightarrow$ Stores the non-zero memory address directly.
- `None` $\rightarrow$ Encoded as all zeros (`0x0`).

```rust
use std::mem::size_of;

// A reference is 8 bytes
assert_eq!(size_of::<&String>(), 8);

// Option<&String> is ALSO 8 bytes (0 discriminant overhead!)
assert_eq!(size_of::<Option<&String>>(), 8);

// Option<u32> requires a tag because u32 has no invalid bit patterns
assert_eq!(size_of::<u32>(), 4);
assert_eq!(size_of::<Option<u32>>(), 8); // 4 bytes payload + 1 byte tag + 3 bytes padding
```

---

## 4. Making Illegal States Unrepresentable

Enums enforce domain invariants at compile time, eliminating invalid runtime states that product types permit.

### Comparison: Struct vs. Enum for Result Representation

```rust
// Anti-Pattern: Product Type with Optional Fields
struct Result1<T, E> {
    ok: Option<T>,
    err: Option<E>,
}

// Idiomatic: Sum Type
enum Result2<T, E> {
    Ok(T),
    Err(E),
}
```

### State Space Analysis

| Representation | Possible States | Valid States | Invalid / Impossible States |
| :--- | :--- | :--- | :--- |
| `Result1<T, E>` | $2 \times 2 = 4$ | `{Some, None}`, `{None, Some}` | `{None, None}` (no outcome), `{Some, Some}` (both succeeded and failed) |
| `Result2<T, E>` | $1 + 1 = 2$ | `Ok(T)`, `Err(E)` | **Zero** (Illegal states are unrepresentable) |

---

## 5. The `Option<T>` Enum

Rust eliminates `null` pointer exceptions by encoding optional presence into the standard library enum:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

### Reference Push-Down (`&Option<T>` $\rightarrow$ `Option<&T>`)
When matching against a borrowed option (`match &opt`), the compiler automatically "pushes down" the reference from the container to the payload:

```rust
let opt: Option<String> = Some(String::from("hello"));

// Matching on &opt borrows the payload as `&String` without moving ownership
match &opt {
    Some(s) => println!("Borrowed string: {s}"), // `s` has type `&String`
    None => {}
}

// `opt` remains fully valid and owned:
println!("opt is still intact: {opt:?}");
```

---

## 6. Pattern Matching with `match`

### 6.1 Exhaustiveness Checking
The `match` construct requires every possible variant to be handled. Omitting a variant results in compile-time error `E0004`:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state}");
            25
        }
    }
}
```

### 6.2 Catch-All Patterns and Placeholders
- **Value binding catch-all**: `other => handle(other)` (binds the unmatched value).
- **Discard placeholder**: `_ => ()` (matches anything without variable binding).

---

## 7. Concise Control Flow: `if let` and `let else`

### 7.1 `if let` (Single Variant Matching)
Replaces boilerplate `match` expressions with wildcard fallbacks:

```rust
let config_max = Some(3u8);

// Concise equivalent of `match config_max { Some(max) => println!("{max}"), _ => () }`
if let Some(max) = config_max {
    println!("Maximum: {max}");
}
```

### 7.2 `if let ... else`
Handles a primary variant versus all alternative cases:

```rust
if let Coin::Quarter(state) = coin {
    println!("Quarter from {state}");
} else {
    println!("Non-quarter coin");
}
```

### 7.3 `let else` (Guard Clauses / Early Exit)
Binds the variant payload if matching succeeds; diverges (`return`, `break`, `continue`, `panic!`) on mismatch:

```rust
fn process_number(opt: Option<u32>) -> u32 {
    // If Some(n), `n` is extracted into the outer scope.
    // If None, the `else` block MUST diverge.
    let Some(n) = opt else {
        return 0;
    };

    n * 2 // `n` is directly available in the outer scope
}
```
