#  Scopes, Mutability, and Variable Shadowing in Rust

A deep dive into how Rust manages variable lifetimes, scopes, immutability, and variable shadowing.

---

## 1. What is a Scope?

A **scope** is the range within a program for which an item is valid.
In Rust, a scope is typically bounded by curly braces `{ ... }`.

```rust
fn main() {
    // outer_val comes into scope
    let outer_val = 100;

    {
        // inner_val comes into scope
        let inner_val = 200;
        println!("Inner: {outer_val} and {inner_val}");
    } // inner_val goes out of scope and its memory is cleaned up

    println!("Outer: {outer_val}");
    // println!("{inner_val}"); //  ERROR: cannot find value `inner_val` in this scope
}
```

---

## 2. Mutability (`mut`) vs. Shadowing (`let`)

### Key Distinction:
- **`let mut x = ...`**: You are modifying the **value** stored inside the **existing** variable. The variable's type can **never** change.
- **`let x = ...` (Shadowing)**: You are creating a **brand new variable** that happens to reuse the name `x`. The new variable can have a **different type** and is **immutable** by default unless explicitly declared with `let mut`.

```rust
// Example 1: Mutability
let mut count = 5;
count = 6; //  Value changed
// count = "six"; //  Type mismatch error: expected integer, found &str

// Example 2: Shadowing
let count = 5;
let count = count + 1; //  New variable: count is 6
let count = "six";     //  New variable with DIFFERENT TYPE: count is &str
```

---

## 3. Nested Scopes & Shadowing Walkthrough

Let's trace the exact challenge problem from our practice session:

```rust
fn main() {
    let x = 5;
    let x = x + 1; // x is shadowed to 6

    let y = {
        let x = x * 2; // x in this block is 6 * 2 = 12
        let z = {
            let x = "rust"; // x in this innermost block is &str "rust"
            x.len()         // evaluates to usize 4
        };
        x + (z as i32)      // 12 + 4 = 16
    }; // Block ends: y = 16, inner x variables dropped

    let mut x = y; // New binding: x is 16 and is mutable

    {
        let x = x * 2; // Inner shadow: x is 32 (immutable in this scope)
        println!("Scope A: x = {x}"); // Prints: Scope A: x = 32
    } // Inner x dropped! Outer x is still 16

    x += 10; // Outer x was mutable: 16 + 10 = 26
    println!("Scope B: x = {x}, y = {y}"); // Prints: Scope B: x = 26, y = 16
}
```

---

## 4. Common Compiler Pitfalls

### Pitfall 1: Reassigning an Immutable Variable
```rust
let x = 5;
x = 10; //  [E0384]: cannot assign twice to immutable variable `x`
```
*Fix:* Use `let mut x = 5;` or shadow with `let x = 10;`.

### Pitfall 2: Reassigning Without `let` After Inner Scope
```rust
let x = 5;
{
    let x = 10;
}
x = 20; //  [E0384]: Outer `x` was never mutable!
```

### Pitfall 3: Trying to change type on `mut`
```rust
let mut buffer = "   ";
buffer = buffer.len(); //  [E0308]: mismatched types: expected `&str`, found `usize`
```
*Fix:* Use shadowing: `let buffer = buffer.len();`.
