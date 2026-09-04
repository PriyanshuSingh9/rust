# Chapter 10.2: Traits: Defining Shared Behavior

This notebook provides a complete first-principles analysis of Chapter 10.2 of the Rust Book, detailing trait definitions, coherence and the orphan rule, default method delegation, trait bounds, `where` clauses, `impl Trait` return limits, conditional method implementations, and blanket implementations.

---

## 1. What Are Traits?

A **trait** defines a set of method signatures that describe a common contract or capability. A type's behavior consists of the methods that can be called on it; different types share the same behavior if they can be invoked with the same method signatures.

```text
               ┌────────────────────────────────────────────────────────┐
               │                     TRAIT: Summary                     │
               │         fn summarize(&self) -> String;                 │
               └───────────────┬────────────────────────┬───────────────┘
                               │                        │
                               ▼                        ▼
               ┌────────────────────────┐   ┌────────────────────────┐
               │    struct NewsArticle  │   │    struct SocialPost   │
               ├────────────────────────┤   ├────────────────────────┤
               │ impl Summary for ...   │   │ impl Summary for ...   │
               │ Headline + Author + Loc│   │ Username + Content     │
               └────────────────────────┘   └────────────────────────┘
```

### The Trait Scope Rule
To call methods defined by a trait on an instance, **the trait itself must be brought into scope** via `use` (e.g., `use traits::Summary;`). If the trait is not in scope, its methods are not available on the type, even if the type itself is in scope.

---

## 2. Coherence and the Orphan Rule

Rust strictly enforces **Coherence**: there can only ever be **one** implementation of a trait for any given type throughout the entire dependency graph.

### The Orphan Rule Invariant
> **You can implement a trait on a type if and only if at least one of them (either the trait OR the type) is local to your crate.**

```text
Trait Location       Type Location        Can You Implement It?
─────────────────────────────────────────────────────────────────
Local crate          Local crate          YES (e.g. Summary for SocialPost)
Local crate          External crate       YES (e.g. Summary for Vec<T>)
External crate       Local crate          YES (e.g. Display for SocialPost)
External crate       External crate       NO! (e.g. Display for Vec<T> - REJECTED)
```

### Why the Orphan Rule Exists
Without this rule, two independent third-party crates (Crate A and Crate B) could both implement `Display` for `Vec<T>`. If a downstream binary imported both Crate A and Crate B, the compiler would face an unresolvable ambiguity: which `Display` implementation should be invoked? The orphan rule guarantees that other crates cannot break your code, and your code cannot break theirs.

---

## 3. Default Implementations & Method Delegation

Traits can provide default method implementations that implementors may choose to keep or override:

```rust
pub trait Summary {
    // Required method (no default body):
    fn author(&self) -> String;

    // Default method that delegates to the required method:
    fn summarize_author(&self) -> String {
        format!("Read more from {}", self.author())
    }
}
```

- **Method Delegation**: A default implementation can invoke other methods defined in the same trait, even if those methods lack default implementations.
- **Implementor Ergonomics**: Implementors only need to define `author(&self)`, and automatically inherit `summarize_author(&self)`.

---

## 4. Traits as Parameters: `impl Trait` vs. Trait Bounds

There are two primary syntaxes for taking parameters that implement traits:

### 4.1 `impl Trait` Syntax Sugar
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
Convenient and concise for simple cases.

### 4.2 Trait Bound Syntax
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### 4.3 Expressive Power Difference (Crucial Invariant)
The two syntaxes are not always equivalent when multiple parameters are involved:

```rust
// Allows item1 and item2 to have DIFFERENT concrete types
// (e.g., item1 is NewsArticle, item2 is SocialPost):
pub fn notify(item1: &impl Summary, item2: &impl Summary)

// Forces item1 and item2 to have the EXACT SAME concrete type:
pub fn notify<T: Summary>(item1: &T, item2: &T)
```

---

## 5. Multiple Bounds & `where` Clauses

### Multiple Trait Bounds (`+` Syntax)
Constrain a type to implement multiple traits simultaneously:

```rust
pub fn notify<T: Summary + Display>(item: &T) { ... }
```

### `where` Clauses
When multiple generic parameters each have multiple bounds, inline signatures become difficult to parse. Rust provides `where` clauses to place bounds after the return type:

```rust
// Hard to read:
fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32

// Idiomatic with where clause:
fn some_fn<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
```

---

## 6. Returning Types that Implement Traits (`-> impl Trait`)

Functions can return `impl Trait` to return a value by its capability interface without exposing the underlying concrete type:

```rust
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("content"),
        reply: false,
        repost: false,
    }
}
```

### The Single Concrete Type Limitation
You can only use `impl Trait` if the function returns a **single concrete type** across all control-flow paths.

```rust
// COMPILER ERROR [E0308]: mismatched types
fn returns_summarizable_branch(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { ... } // returns NewsArticle
    } else {
        SocialPost { ... }  // returns SocialPost: REJECTED!
    }
}
```
- **Why**: Rust monomorphizes `impl Trait` return types statically at compile time. It must know the exact memory layout (stack size) of the returned value.
- **Solution**: To return heterogeneous types based on runtime conditions, use **Trait Objects** with dynamic dispatch: `Box<dyn Summary>` (covered in Chapter 17).

---

## 7. Conditional Method Implementation & Blanket Implementations

### 7.1 Conditional Method Implementation via Trait Bounds
You can use trait bounds on an `impl` block to add methods only to instances where the generic type parameter satisfies specific traits:

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// `cmp_display` ONLY exists if T implements Display AND PartialOrd:
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest: x = {}", self.x);
        } else {
            println!("Largest: y = {}", self.y);
        }
    }
}
```

### 7.2 Blanket Implementations
A **blanket implementation** conditionally implements a trait for *any* type that implements another trait.

In the standard library:
```rust
impl<T: Display> ToString for T {
    // Automatically gives `.to_string()` to ANY type that implements Display!
}
```
This is why implementing `Display` on your struct automatically grants it `.to_string()` without writing a single line of extra code.
