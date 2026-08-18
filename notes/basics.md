# 🦀 Rust Chapter 3: Common Programming Concepts — Complete Reference

This notebook summarizes all key concepts, mental models, syntax rules, and pitfalls covered across Chapters 1 to 3 of the Rust Book.

---

## 1. Variables, Mutability, and Constants

### Immutability by Default
By default, variables in Rust are **immutable**. Once bound to a value, you cannot reassign them.
```rust
let x = 5;
// x = 6; // ❌ ERROR: cannot assign twice to immutable variable `x`
```

### Mutability (`mut`)
Using `mut` allows reassigning a variable's **value**, but **never its type**.
```rust
let mut x = 5;
x = 6; // ✅ Allowed
// x = "six"; // ❌ ERROR: mismatched types (expected integer, found &str)
```

### Constants (`const`)
- Declared with `const` and **must** have an explicit type annotation.
- Can be placed in global/module scope.
- Must evaluate to a compile-time constant expression, not a runtime calculation.
- Constant naming convention: `SCREAMING_SNAKE_CASE`.
```rust
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
```

---

## 2. Scopes and Variable Shadowing

### Shadowing (`let x = ...` repeated)
Shadowing allows you to re-declare a variable with the same name.
- **Key Advantage 1:** You can change the variable's **type** while reusing a descriptive name.
- **Key Advantage 2:** The new variable remains **immutable** after the transformation.

```rust
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize (type changed cleanly!)
```

### Scope Blocks (`{ ... }`)
- Blocks create a new local scope.
- Variables defined in an inner scope shadow outer variables without modifying the outer variable.
- When an inner scope ends, inner variables are dropped, restoring the outer binding.

```rust
let x = 5;
{
    let x = x * 2; // Shadows outer x: x is 10 here
    println!("Inner x: {x}"); // 10
}
println!("Outer x: {x}"); // 5 (outer x was unchanged!)
```

| Feature | `let` (Immutable) | `let mut` (Mutable) | Shadowing (`let`) |
| :--- | :--- | :--- | :--- |
| **Can change value?** | ❌ No | ✅ Yes (`x = 6`) | ✅ Yes (new binding) |
| **Can change type?** | ❌ No | ❌ No | ✅ Yes (`let x = "str"`) |
| **Remains immutable?**| ✅ Yes | ❌ No | ✅ Yes (unless `let mut`) |

---

## 3. Data Types

Rust is **statically typed**; the compiler must know or infer all types at compile time.

### Scalar Types (Single Values)
1. **Integers:**
   - Signed: `i8`, `i16`, `i32` (default), `i64`, `i128`, `isize` (pointer-sized).
   - Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (indices/sizes).
   - **Integer Overflow:** In `debug` builds, overflow causes a runtime panic. In `--release` builds, Rust performs two's complement wrapping.
2. **Floating-Point:** `f32`, `f64` (default; provides high precision with near-identical speed on modern 64-bit CPUs).
3. **Booleans:** `bool` (`true` or `false`).
4. **Characters:** `char` (4 bytes; represents a Unicode Scalar Value, e.g., `'a'`, `'😻'`, `'中'`). Uses single quotes `' '`.

### Compound Types (Multiple Values)
1. **Tuples:**
   - Group values of **different** types with fixed size.
   - Access via destructuring (`let (x, y) = tup;`) or dot notation (`tup.0`, `tup.1`).
   - The empty tuple `()` is called the **unit type** (representing no value / void).
2. **Arrays:**
   - Store multiple values of the **same** type with fixed size on the **stack**.
   - Type signature: `[type; length]`, e.g., `let a: [i32; 5] = [1, 2, 3, 4, 5];`.
   - Repeated initial value: `let a = [3; 5];` (produces `[3, 3, 3, 3, 3]`).
   - Out-of-bounds indexing (e.g. `a[10]`) causes a **panic** at runtime, preserving memory safety.

---

## 4. Functions & Expression-Oriented Design

### Statements vs Expressions
- **Statements:** Instructions that perform an action and do **not** return a value (e.g., `let x = 6;`).
  - You *cannot* do `let x = (let y = 6);`.
- **Expressions:** Evaluate to a resultant value (e.g., `5 + 6`, function calls, block `{ ... }`).
  - Expressions do **not** end with a semicolon.

### The Semicolon Rule & The Unit `()` Trap
Adding a semicolon turns an expression into a statement, discarding its value and returning `()`:
```rust
fn add_one(x: i32) -> i32 {
    x + 1 // Expression -> returns i32 ✅
    // x + 1; // Statement -> returns () ❌ (causes mismatched types error [E0308])
}
```

---

## 5. Control Flow

### `if` Expressions
- Condition must evaluate to a strict `bool` (Rust has **no truthy/falsy** values).
- `if` is an expression and can be used on the right-hand side of a `let` statement.
- **Rule:** Every branch arm must return the **exact same type**.

```rust
let condition = true;
let number = if condition { 5 } else { 6 }; // ✅ Both branches return integer
// let number = if condition { 5 } else { "six" }; // ❌ ERROR: incompatible arm types
```

### Loops

#### 1. `loop` (Infinite loop & returning values)
- Infinite loop until `break`.
- Can return values from a loop directly: `break value;`.
- Supports **loop labels** (`'label:`) to target outer loops from nested inner loops.

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // Returns 20 to result
    }
};

'outer: loop {
    loop {
        break 'outer; // Breaks the outer loop cleanly
    }
}
```

#### 2. `while` (Conditional loop)
Runs while the condition evaluates to `true`.
```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
```

#### 3. `for` (Collection traversal & ranges)
- The most idiomatic, safest, and fastest loop construct in Rust.
- Eliminates manual index bounds checks and out-of-bounds panics.

```rust
// Iterating over an array
let arr = [10, 20, 30, 40, 50];
for element in arr {
    println!("{element}");
}

// Ranges and reverse traversal
for number in (1..4).rev() { // 3, 2, 1
    println!("{number}!");
}
```

---

## 6. Lessons Learned from Practice Exercises

| Problem | Key Takeaway / Common Pitfall |
| :--- | :--- |
| **Palindrome Checking** | Return early (`return false;`) on mismatch rather than toggling a variable that could get overwritten in later iterations. |
| **Collatz Sequence** | Update the dynamic tracking state (`current % 2`), not the immutable initial input (`start % 2`). |
| **Twelve Days of Christmas** | Array slicing `[0..day]` excludes `day`. Use inclusive ranges `0..=day` or indexing `(0..=day).rev()`. |
| **Matrix Rotation** | Stack 2D arrays `[[i32; 3]; 3]` map `matrix[row][col]` to `rotated[col][size - 1 - row]`. |
| **Expressions vs Statements** | Semicolons on branch return values turn values into unit `()`. Keep branches consistent and omit trailing semicolons for returns. |
