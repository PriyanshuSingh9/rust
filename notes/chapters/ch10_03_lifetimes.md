# Chapter 10.3: Validating References with Lifetimes

This notebook provides a complete first-principles architectural analysis of Chapter 10.3 of the Rust Book, detailing the borrow checker's scope intersection model, generic lifetime parameters, struct lifetime bounds, compiler elision rules, and the `'static` lifetime.

---

## 1. The Purpose of Lifetimes & The Borrow Checker

In Rust, every reference has a **lifetime**: the duration of execution during which the reference points to valid, allocated memory.

The primary goal of lifetimes is to eliminate **dangling references** (use-after-free bugs and pointer invalidation) at compile time without a garbage collector.

### How the Borrow Checker Evaluates Scopes
The Rust compiler includes a **borrow checker** that compares scopes to ensure that all borrows are valid:

```text
{
    let r;                // ──┐ Lifetime of `r` ('a)
                          //   │
    {                     //   │
        let x = 5;        // ──┼──┐ Lifetime of `x` ('b)
        r = &x;           //   │  │
    }                     // ──┼──┘ `x` is dropped here
                          //   │
    println!("r: {r}");   // ──┘ ERROR: `r` refers to memory of `x` after `x` is dropped!
}
```
Because the lifetime of `r` (`'a`) is larger than the lifetime of `x` (`'b`), the borrow checker rejects the program with `E0597` (*`x` does not live long enough*).

---

## 2. Generic Lifetime Syntax & Core Invariants

Lifetime annotations do **not** change how long any value lives. They describe the relationships between the lifetimes of multiple references so the borrow checker can prove memory safety.

```rust
&i32        // A reference
&'a i32     // An immutable reference with explicit lifetime `'a`
&'a mut i32 // A mutable reference with explicit lifetime `'a`
```

### The Intersection Invariant in Function Signatures
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- When concrete references are passed to `longest`, the generic lifetime `'a` represents the **concrete intersection (the smaller)** of the lifetimes of `x` and `y`.
- The returned reference is guaranteed to be valid for that intersection duration.
- If you return a reference from a function, its lifetime **must** match one of the input parameters. Returning a reference to locally created data triggers `E0515` (*cannot return reference to local variable*).

---

## 3. Lifetime Annotations in Struct Definitions

When a struct stores borrowed data rather than owned data, its definition must include a lifetime annotation:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### Struct Invariant
An instance of `ImportantExcerpt` **cannot outlive** the reference it holds in its `part` field. If the underlying string data is dropped, the struct instance becomes invalid at compile time.

---

## 4. The Three Lifetime Elision Rules

In early Rust (pre-1.0), every reference in a function signature required an explicit lifetime annotation. The Rust team analyzed common patterns and encoded **deterministic heuristics** into the compiler called the **Lifetime Elision Rules**:

```text
               ┌────────────────────────────────────────────────────────┐
               │                LIFETIME ELISION RULES                  │
               └───────────────┬────────────────────────┬───────────────┘
                               │                        │
                               ▼                        ▼
               ┌────────────────────────┐   ┌────────────────────────┐
               │    RULE 1: INPUTS      │   │    RULE 2: OUTPUTS     │
               ├────────────────────────┤   ├────────────────────────┤
               │ Each elided parameter  │   │ If exactly 1 input     │
               │ reference gets its     │   │ lifetime, assign it to │
               │ own distinct lifetime: │   │ all output lifetimes:  │
               │ fn f(x: &i32, y: &i32) │   │ fn f(x: &str) -> &str  │
               │ └──> <'a, 'b>(x, y)    │   │ └──> <'a>(x) -> &'a    │
               └────────────────────────┘   └────────────────────────┘
                               │
                               ▼
               ┌────────────────────────────────────────────────────────┐
               │             RULE 3: METHODS (&self / &mut self)        │
               ├────────────────────────────────────────────────────────┤
               │ If multiple input lifetimes exist, but one is &self or │
               │ &mut self, assign the lifetime of self to all outputs: │
               │ fn method(&self, other: &str) -> &str                  │
               │ └──> output receives lifetime of `&self`               │
               └────────────────────────────────────────────────────────┘
```

If the compiler applies all three rules and any output reference still has an ambiguous lifetime, compilation halts with error `E0106` (*missing lifetime specifier*), requiring explicit manual annotation.

---

## 5. Lifetimes in `impl` Blocks

When implementing methods on a struct with a lifetime parameter:

```rust
impl<'a> ImportantExcerpt<'a> {
    // Rule 1 applies: &self gets a lifetime, return is owned i32
    fn level(&self) -> i32 {
        3
    }

    // Rule 3 applies: output &str automatically receives the lifetime of &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

---

## 6. The Static Lifetime (`'static`)

`'static` is a reserved lifetime name denoting that the affected reference can live for the **entire duration of the program**.

```rust
let s: &'static str = "I have a static lifetime.";
```

- **Storage**: String literals are embedded directly in the compiled binary's read-only data section (`.rodata`). The binary is mapped into memory when the process starts and remains mapped until the process exits.
- **`'static` as a Trait Bound (`T: 'static`)**: Means the type `T` can live for as long as needed without being invalidated by a borrow. All owned types (`i32`, `String`, `Vec<u8>`) satisfy `T: 'static`, even if created dynamically on the heap at runtime.

---

## 7. Unified Syntax: Generics, Trait Bounds, and Lifetimes Together

Combining all Chapter 10 dimensions into a single function signature:

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
