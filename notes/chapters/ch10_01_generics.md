# Chapter 10.1: Generic Data Types and Monomorphization

This notebook provides a complete first-principles analysis of Chapter 10.1 of the Rust Book, detailing generic function deduplication, parameter scope in `impl` blocks, method-level type mixing, and compile-time monomorphization.

---

## 1. The Purpose of Generics

Generics are abstract stand-ins for concrete types. They allow writing algorithms and data structures once while preserving strict compile-time type safety and maximum execution performance.

```text
Without Generics:
largest_i32(list: &[i32]) -> &i32   ──┐
largest_char(list: &[char]) -> &char ─┼─> Duplicate ASTs & logic branches
largest_f64(list: &[f64]) -> &f64   ──┘

With Generics:
largest<T: PartialOrd>(list: &[T]) -> &T ──> Single abstract implementation
```

---

## 2. Generic Functions & Capability Invariants

An unconstrained generic type parameter `T` has **no assumed capabilities**:
- It cannot be printed (`Display` / `Debug` required).
- It cannot be cloned or copied (`Clone` / `Copy` required).
- It cannot be compared (`PartialEq` / `PartialOrd` required).

```rust
// Requires PartialOrd to permit the `>` comparison operator:
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### Architectural Takeaway: Returning `&T` vs. `T`
- If `largest` returned an owned `T`, the line `let mut largest = list[0];` would attempt to **move** out of the borrowed slice `list`, triggering compiler error `E0507`.
- Fixing that by returning `T` would force adding the `T: Copy` trait bound, restricting the function solely to stack-copyable types.
- Returning a borrowed reference `&T` allows the function to operate efficiently across both stack-allocated primitives (`i32`) and heap-allocated types (`String`, `Vec<u8>`) without heap cloning.

---

## 3. Generic Structs & Enums

### 3.1 Single vs. Multiple Type Parameters

```rust
// Forces x and y to be of the exact same concrete type:
struct PointHomogeneous<T> {
    x: T,
    y: T,
}

// Allows x and y to be of different concrete types:
struct PointHeterogeneous<T, U> {
    x: T,
    y: U,
}
```

### 3.2 Canonical Standard Library Enums

```rust
// Represents optional absence without null pointers:
enum Option<T> {
    Some(T),
    None,
}

// Represents recoverable fallibility with distinct success/error types:
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

## 4. Generic Method Definitions & Parameter Scopes

### 4.1 Declaring Parameters on `impl`
To implement methods on a generic struct, declare the generic type parameters immediately after `impl`:

```rust
impl<T, U> Point<T, U> {
//   ^^^^      ^^^^^^^^
//   Declares  Names the type being extended
    fn x(&self) -> &T {
        &self.x
    }
}
```
If `<T, U>` is omitted after `impl`, the compiler treats `Point<T, U>` as looking for concrete types named `T` and `U`.

### 4.2 Concrete Type Specialization
Methods can be implemented exclusively for specific concrete types without generic parameters on `impl`:

```rust
impl Point<f32, f32> {
    // Only available on instances of Point<f32, f32>:
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

#### Specialization Invariant (No OOP-style Method Overriding)
- You **cannot** simultaneously implement specific and generic methods of the same name.
  For example, implementing a general `distance_from_origin` on `Point<T, U>` and a specific one on `Point<f32, f32>` triggers a compiler error because Rust does not know which implementation to invoke, lacking OOP-style dynamic method overriding.
- **No Universal "Root" Object**: Unlike Java or C# where every type inherits methods from a base class (like `Object.toString()`), Rust has no universal root object or default methods. An unconstrained generic type `T` cannot be printed via `println!("{x}")` or cloned because `T` has no assumed capabilities without explicit trait bounds.

```rust
// Rejected by compiler: T has no Display trait bound
fn print_slice<T>(v: &[T]) {
    for x in v {
        println!("{x}"); // ERROR: cannot format with the default formatter
    }
}
```

### 4.3 Method-Level Generic Parameters (`mixup`)
Methods can introduce generic parameters that are completely distinct from the struct's definition:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    // X2 and Y2 are scoped strictly to the `mixup` method:
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

---

## 5. Monomorphization: Zero-Cost Static Dispatch

Rust implements generics via **Monomorphization** at compile time.

```text
Source Code (Generics):
let integer_pt = Point { x: 5, y: 10 };
let float_pt   = Point { x: 1.0, y: 4.0 };

                   │
                   ▼ Monomorphization (Compiler Phase)
Generated Concrete Machine Code:
struct Point_i32 { x: i32, y: i32 }
struct Point_f64 { x: f64, y: f64 }
```

### Systems Engineering Characteristics

| Characteristic | Rust Generics (Monomorphization) | Dynamic / Object-Oriented Generics (Java / Python) |
| :--- | :--- | :--- |
| **Dispatch Mechanism** | **Static Dispatch**: Direct CPU instruction jumps. | **Dynamic Dispatch**: Pointer dereferences through vtables or type objects. |
| **Runtime Overhead** | **Zero runtime cost**: Identical to hand-written concrete code. | Pointer indirection, branch misprediction penalties. |
| **Inlining** | Aggressively inlineable by LLVM. | Inhibited by dynamic dispatch boundaries. |
| **Binary Size** | Potential **code bloat** (binary size grows per concrete type). | Single shared bytecode / machine code representation. |
| **Instruction Cache** | May cause instruction cache pressure with many distinct type instantiations. | Smaller code footprint, potentially higher I-cache locality. |
