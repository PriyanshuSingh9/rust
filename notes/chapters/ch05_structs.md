# Chapter 5: Structs, Methods, and Ownership Permissions

This notebook provides a complete first-principles analysis of Chapter 5 of the Rust Book, focusing on custom data structures, method syntax, and the compile-time ownership permissions model governing methods.

---

## 1. Struct Fundamentals & Memory Layout

Structs group related data into named, strongly-typed records on the stack.

### Types of Structs

```rust
// 1. Classic Named-Field Struct
struct Rectangle {
    width: u32,
    height: u32,
}

// 2. Tuple Struct (fields accessed via index .0, .1)
struct Point(i32, i32);
struct Color(u8, u8, u8);

// 3. Unit-Like Struct (zero memory footprint, useful for trait markers)
struct AlwaysEqual;
```

### Memory Layout
For a classic struct whose fields are primitive scalars (e.g., `u32`, `i32`), the struct is laid out contiguously on the stack with field padding determined by alignment rules:

```text
Stack Frame (Rectangle):
+---------------+---------------+
| width (4 B)   | height (4 B)  |  => Total: 8 bytes
+---------------+---------------+
```

---

## 2. Method Syntax (`impl` Blocks)

Methods are functions defined inside an `impl` block whose first parameter is a variation of `self`.

```rust
impl Rectangle {
    // Immutable borrow: reads state without mutation or ownership transfer
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Mutable borrow: mutates struct fields in place
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Value consumption: takes ownership of instance (moves it)
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // Associated Function (not a method): lacks a `self` parameter
    // Acts as a constructor, called with `Rectangle::square(40)`
    fn square(side: u32) -> Self {
        Rectangle { width: side, height: side }
    }
}
```

### Automatic Referencing and Dereferencing
When invoking `rect.area()`, Rust automatically inserts `&`, `&mut`, or `*` to match the method signature:
- `rect.area()` is automatically rewritten by the compiler to `(&rect).area()`.
- Unlike C/C++, Rust eliminates the syntactic distinction between `.` and `->`.

---

## 3. Methods and the Permissions Model (R / W / O)

Every variable binding and path possesses three compile-time permissions:
- **Read (R)**: Permission to inspect or copy data.
- **Write (W)**: Permission to mutate in-place.
- **Own (O)**: Permission to move or drop data.

### Receiver Permission Requirements

| Receiver Form | Desugared Form | Required Permissions on Path | Effect on Caller |
| :--- | :--- | :--- | :--- |
| `&self` | `self: &Self` | **Read (R)** | Borrows immutably; caller retains R |
| `&mut self` | `self: &mut Self` | **Read (R) + Write (W)** | Borrows mutably; caller loses R and W until borrow ends |
| `self` | `self: Self` | **Read (R) + Own (O)** | Moves value; caller loses all permissions (unless `Copy`) |

### Implicit Reborrowing
Calling an `&self` method on a mutable reference (`&mut Rectangle`) compiles cleanly because Rust implicitly reborrows the `&mut` reference as an immutable `&` reference:

```rust
let mut rect = Rectangle { width: 10, height: 20 };
let rect_ref: &mut Rectangle = &mut rect;

// Desugars to Rectangle::area(&*rect_ref)
println!("{}", rect_ref.area()); 
```

---

## 4. Moves with `self` and Compiler Invariants

### 4.1 Loss of Ownership on By-Value Calls
Calling a by-value `self` method consumes the instance:

```rust
let rect = Rectangle { width: 10, height: 20 };
let other = Rectangle { width: 15, height: 25 };

let max_rect = rect.max(other);

// ERROR: borrow of moved value `rect` (if Rectangle is not Copy)
// println!("{}", rect.area());
```

### 4.2 The `cannot move out of *self` Error (`E0507`)

Consider a method attempting to update a mutable reference with a by-value method:

```rust
impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        // ERROR: cannot move out of `*self` which is behind a mutable reference
        *self = self.max(other);
    }
}
```

#### Why Rust Rejects This (The Double-Free Proof)
Even though `Rectangle` with `u32` fields appears safe to copy on the stack, Rust's borrow checker rejects moving out of `*self` because it cannot assume the type is free of heap allocations:

If `Rectangle` contains a heap-allocated field (like `name: String`):
1. `self.max(other)` would take ownership of `*self` (including `r1.name`).
2. Inside `max`, `r1.name` and `r2.name` are consumed and deallocated when `max` returns.
3. Returning to `set_to_max`, `*self` now contains a deallocated pointer (use-after-free).
4. Assigning `*self = max` triggers an implicit `drop(*self)` on the old value, attempting to free `r1.name` a second time (**Double-Free Undefined Behavior**).

```text
Memory Timeline of Bad Move:
[Stack: r1.name] ---> [Heap: "r1" (Allocated)]
                      |
                      v `self.max(other)` consumes and drops heap string
                      [Heap: "r1" (FREED / Dangling Pointer)]
                      |
                      v `*self = max` drops old r1 before overwriting
                      [Heap: "r1" (DOUBLE FREE -> SIGSEGV / UB)]
```

---

## 5. Resolving Move Restrictions: Explicit `Copy` Semantics

To allow moving out of `*self` behind a reference, the type must explicitly implement `Copy` and `Clone`:

```rust
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_to_max(&mut self, other: Rectangle) {
        // Safe: desugars to Rectangle::max(*self, other), where `*self` performs a bitwise copy
        *self = self.max(other);
    }
}
```

### Why Rust Does Not Auto-Derive `Copy`
Rust deliberately requires explicit `#[derive(Copy)]` annotations for API stability:
- If `Copy` were implicit for all all-scalar structs, adding a heap-allocated field (`name: String`) in a library update would silently break all downstream client code relying on `Copy` semantics.
- Explicit `derive(Copy)` acts as a formal contract that the type will remain a pure bitwise copyable value.

---

## 6. Field-Level Borrowing Conflicts

When a method borrows `&mut self` to expose a field reference, the entire struct instance is mutably borrowed:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x(); // Borrows ALL of `p` mutably
    *x += 1;

    // ERROR: cannot borrow `p.y` as immutable because `p` is borrowed as mutable
    // println!("{} {}", *x, p.y); 

    // OK: accessing fields directly without method encapsulation allows disjoint borrowing:
    let x_direct = &mut p.x;
    *x_direct += 1;
    println!("{} {}", p.x, p.y); // Allowed because Rust tracks field disjointness
}
```
