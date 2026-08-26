# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

This notebook provides a complete first-principles analysis of Chapter 7 of the Rust Book, detailing compilation boundaries, the module system, privacy rules, path resolution, and multi-file project architecture.

---

## 1. Packages, Crates, and Compilation Boundaries

| Entity | Definition | Ruled By | Maximum Per Package |
| :--- | :--- | :--- | :--- |
| **Package** | A directory on disk containing a `Cargo.toml` manifest that describes how to build one or more crates. | **Cargo** | N/A |
| **Crate** | The smallest compilation unit evaluated by `rustc` at one time, compiling into a single binary executable or `.rlib` library. | **`rustc`** | Unlimited binaries, at most 1 library |
| **Module** | An internal organizational unit inside a crate that controls scoping and privacy. | **Language syntax (`mod`)** | Unlimited |

### Binary Crates vs. Library Crates

```text
               ┌────────────────────────────────────────────────────────┐
               │                         PACKAGE                        │
               │                   (1 Cargo.toml file)                  │
               └───────────────┬────────────────────────┬───────────────┘
                               │                        │
                               ▼                        ▼
               ┌────────────────────────┐   ┌────────────────────────┐
               │     LIBRARY CRATE      │   │     BINARY CRATES      │
               ├────────────────────────┤   ├────────────────────────┤
               │ • At most ONE /package │   │ • UNLIMITED /package   │
               │ • Root: src/lib.rs     │   │ • Root: src/main.rs    │
               │ • NO fn main()         │   │ • Extra: src/bin/*.rs  │
               │ • Exposes public API   │   │ • MUST have fn main()  │
               └────────────────────────┘   └────────────────────────┘
```

---

## 2. The Module Tree Architecture

Modules organize code inside a crate into hierarchical namespaces.

```text
crate (src/lib.rs)
 ├── front_of_house
 │    ├── hosting
 │    │    ├── add_to_waitlist
 │    │    └── seat_at_table
 │    └── serving
 │         ├── take_order
 │         ├── serve_order
 │         └── take_payment
 └── back_of_house
      ├── Breakfast (struct)
      ├── Appetizer (enum)
      └── deliver_order
```

### Module Declaration Rules
1. **Inline Module**: `mod front_of_house { /* code */ }`
2. **File Module**: `mod front_of_house;` tells the compiler to load `src/front_of_house.rs` (or `src/front_of_house/mod.rs` in legacy 2015 edition).
3. **Submodule File**: Inside `src/front_of_house.rs`, writing `pub mod hosting;` tells the compiler to load `src/front_of_house/hosting.rs`.

---

## 3. Privacy Rules and Encapsulation Invariants

By default, all items in Rust (functions, methods, structs, enums, modules, and fields) are **private to their immediate parent module**.

### The Two Core Privacy Rules
1. **Child Access to Ancestors**: An item inside a child module can always see and use private items declared in any of its enclosing ancestor modules.
2. **Parent Access to Children**: An item in a parent module **cannot** access private items inside its child modules unless the child item is explicitly marked with `pub`.

```rust
mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        // Allowed: `super` resolves to the parent module (crate root),
        // and children can access parent items.
        super::deliver_order();
    }
}

fn deliver_order() {}
```

---

## 4. Struct Field Privacy vs. Enum Variant Privacy

Rust treats privacy on `struct` fields and `enum` variants differently based on their invariant guarantees:

### 4.1 Struct Field Privacy (Private by Default)
Marking a struct with `pub` makes the struct type public, but its **fields remain private by default**:

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // Public: readable & writable outside module
        seasonal_fruit: String, // Private: inaccessible outside `back_of_house`
    }

    impl Breakfast {
        // A public constructor is REQUIRED because `seasonal_fruit` is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```
- **Rationale**: Structs often maintain internal invariants (e.g. non-empty buffers, valid ranges). Private fields prevent external code from directly constructing invalid structs with struct literal syntax.

### 4.2 Enum Variant Privacy (Public by Default)
Marking an `enum` with `pub` automatically makes **all of its variants public**:

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,  // Automatically public
        Salad, // Automatically public
    }
}
```
- **Rationale**: An enum's primary purpose is exhaustive pattern matching. Hiding variants would break match statements across module boundaries.

---

## 5. Paths and the `use` Keyword

### 5.1 Absolute vs. Relative Paths
- **Absolute Path**: Starts from the crate root using `crate::` (e.g., `crate::front_of_house::hosting::add_to_waitlist();`).
- **Relative Path**: Starts from the current module scope (e.g., `front_of_house::hosting::add_to_waitlist();`) or uses `super::` to navigate upward.

### 5.2 Idiomatic `use` Conventions
- **Functions**: Bring the **parent module** into scope to make it clear that the function is not defined locally:
  ```rust
  use crate::front_of_house::hosting;
  hosting::add_to_waitlist(); // Idiomatic
  ```
- **Structs, Enums, and Constants**: Bring the **item directly** into scope:
  ```rust
  use std::collections::HashMap;
  let mut map = HashMap::new(); // Idiomatic
  ```

### 5.3 Disambiguation with `as`
When bringing two items with the same name into scope, provide a local alias with `as`:

```rust
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
```

### 5.4 Re-Exporting Names with `pub use`
By default, names brought into scope with `use` are private to that scope. Combining `pub` with `use` exposes the item to external callers under a cleaner namespace:

```rust
// In src/lib.rs:
pub use crate::front_of_house::hosting;

// External crates can now call:
// `restaurent::hosting::add_to_waitlist()` instead of traversing internal tree.
```

### 5.5 Nested Paths and the Glob Operator

```rust
// Combining common prefixes:
use std::{cmp::Ordering, io};

// Self import (imports `std::io` AND `std::io::Write`):
use std::io::{self, Write};

// Glob operator (imports all public items - use sparingly):
use std::collections::*;
```

---

## 6. Multi-File Module File System Hierarchy (2018/2024 Edition)

Modern Rust uses a file-per-module naming convention that mirrors the module hierarchy without needing repetitive `mod.rs` files:

```text
restaurent/
├── Cargo.toml
└── src/
    ├── lib.rs                  # mod front_of_house;
    ├── front_of_house.rs       # pub mod hosting; pub mod serving;
    └── front_of_house/
        ├── hosting.rs          # pub fn add_to_waitlist() {}
        └── serving.rs          # pub fn take_order() {}
```
