# Rust Attributes and Derived Traits Reference

This document provides a first-principles guide to attributes in Rust, covering syntax, compile-time mechanics, derived traits, and common attribute categories.

---

## 1. Core Concept & Syntax

Attributes are metadata attached to Rust source elements (crates, modules, structs, enums, functions, fields, and statements) that instruct the compiler to:
- Generate code (e.g., procedural derive macros).
- Control compiler warnings and diagnostics (lints).
- Conditionally include or exclude code blocks (`cfg`).
- Configure testing harness entry points (`test`).
- Specify low-level memory layout, optimization hints, and FFI rules (`repr`, `inline`, `no_mangle`).

### Inner vs. Outer Attributes

| Syntax | Type | Target Scope | Typical Location |
| :--- | :--- | :--- | :--- |
| `#[attribute]` | **Outer** | Applies to the item immediately following it | Structs, enums, functions, fields |
| `#![attribute]` | **Inner** | Applies to the enclosing container/item | Crate roots (`main.rs`, `lib.rs`), top of modules |

#### Example
```rust
// Applied to the entire crate (inner attribute)
#![deny(unsafe_code)]
#![allow(unused_imports)]

// Applied to the struct directly below (outer attribute)
#[derive(Debug)]
pub struct Rectangle {
    // Applied to a single field (outer attribute)
    #[allow(dead_code)]
    pub width: u32,
    pub height: u32,
}
```

---

## 2. Derive Macros and Trait Implementations

### What `#[derive(...)]` Does
The `#[derive]` attribute invokes procedural macro generators at compile time to automatically implement supported traits for `struct` and `enum` types.

Without `derive`, boilerplate trait implementations must be written manually:

```rust
// Manual implementation:
impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rectangle")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

// Equivalent automated implementation:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

### Standard Derivable Traits

| Trait | Purpose | Requirement on Fields |
| :--- | :--- | :--- |
| `Debug` | Formats output via `{:?}` / `{:#?}` and `dbg!()` | All fields must implement `Debug` |
| `Clone` | Explicit duplicate creation via `.clone()` | All fields must implement `Clone` |
| `Copy` | Implicit bitwise copy semantics on assignment/pass | All fields must implement `Copy` |
| `PartialEq`, `Eq` | Equality comparison (`==`, `!=`) | All fields must implement `PartialEq` / `Eq` |
| `PartialOrd`, `Ord` | Ordering comparison (`<`, `>`, `<=`, `>=`) | All fields must implement `PartialOrd` / `Ord` |
| `Hash` | Hash computation for `HashSet` and `HashMap` keys | All fields must implement `Hash` |
| `Default` | Constructor fallback via `Type::default()` | All fields must implement `Default` |

---

## 3. Formatting Traits: `Display` vs. `Debug`

Rust enforces a strict separation between user-facing output and developer-facing diagnostic output.

| Trait | Format Specifier | Derivable? | Primary Audience |
| :--- | :--- | :--- | :--- |
| `std::fmt::Display` | `{}` | No (manual `impl` required) | End users (clean, domain-specific presentation) |
| `std::fmt::Debug` | `{:?}` (inline), `{:#?}` (pretty) | Yes (`#[derive(Debug)]`) | Developers (debugging, inspecting internal state) |

### Debug Output Variants

```rust
let rect = Rectangle { width: 30, height: 50 };

// Standard debug formatting (single line)
println!("{rect:?}");
// Output: Rectangle { width: 30, height: 50 }

// Pretty-printed debug formatting (multiline with indentation)
println!("{rect:#?}");
// Output:
// Rectangle {
//     width: 30,
//     height: 50,
// }

// dbg! macro: prints expression, file location, line number, and returns ownership
let rect = dbg!(Rectangle { width: 30, height: 50 });
```

---

## 4. Compiler Dead-Code Analysis Interaction

When fields are only read by a derived `Debug` implementation, the compiler still emits a dead-code warning:

```text
warning: fields `width` and `height` are never read
  = note: `Rectangle` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default
```

### Why This Happens
The dead-code lint is designed to catch fields that exist in memory but serve no functional purpose in program logic. Because `#[derive(Debug)]` unconditionally accesses every field to construct a debug string, counting it as a "read" would render dead-code detection completely ineffective for any struct marked with `#[derive(Debug)]`.

---

## 5. Major Attribute Categories

### 5.1 Lint Controls
Controls compiler diagnostics across four severity levels: `allow`, `warn`, `deny`, and `forbid`.

```rust
// Suppress unused code warning for an experimental item
#[allow(dead_code)]
fn experimental_feature() {}

// Upgrade non-snake-case names from warning to hard error
#[deny(non_snake_case)]
fn BadName() {}

// Forbid unsafe code permanently (cannot be overridden in submodules)
#![forbid(unsafe_code)]
```

### 5.2 Test Harness
Directs `cargo test` execution, failure expectations, and execution filtering.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_dimensions() {
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "dimension must be positive")]
    fn test_zero_dimension() {
        panic!("dimension must be positive");
    }

    #[test]
    #[ignore = "expensive network test"]
    fn test_remote_sync() {
        // Skipped unless `cargo test -- --ignored` is passed
    }
}
```

### 5.3 Conditional Compilation (`cfg`)
Includes or excludes code during compilation based on target architecture, OS, or Cargo feature flags.

```rust
// Target OS checks
#[cfg(target_os = "linux")]
fn linux_epoll() {}

#[cfg(windows)]
fn windows_iocp() {}

// Feature-flag checks
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Conditional attribute application (apply derive only if feature is active)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Packet {
    pub payload: Vec<u8>,
}
```

### 5.4 Memory Layout and ABI (`repr`)
Overrides the default compiler struct layout representation.

```rust
// C-compatible layout (disables field reordering)
#[repr(C)]
struct NativeHeader {
    tag: u8,
    size: u32,
}

// Packed representation (zero padding bytes, unaligned fields)
#[repr(packed)]
struct PackedPacket {
    tag: u8,
    val: u32,
}

// Explicit memory alignment (e.g. cache-line aligned to 64 bytes)
#[repr(align(64))]
struct CacheAlignedCounter {
    value: u64,
}
```

### 5.5 Optimization and API Invariants

```rust
// Warns the caller if the returned value is discarded
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub fn make_request() -> Result<(), ()> {
    Ok(())
}

// Inlines across crate boundaries
#[inline]
pub fn fast_path() {}

// Guarantees stable exported C symbol name without compiler name mangling
#[no_mangle]
pub extern "C" fn rust_entrypoint() {}
```

### 5.6 Documentation Comments as Attributes
Rust doc comments are syntactic sugar for `#[doc = "..."]` attributes:

```rust
/// This is a doc comment
pub fn foo() {}

// Equivalent to:
#[doc = " This is a doc comment"]
pub fn foo() {}
```
