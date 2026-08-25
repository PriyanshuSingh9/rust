// Methods, Borrowing, and Ownership Permissions (Chapter 5.3 Deep Dive)
// Demonstrates:
// 1. &self, &mut self, and self (by value)
// 2. R, W, and O permissions model
// 3. Loss of ownership on by-value calls
// 4. Moving out of `*self` and double-free protection
// 5. Why explicit `Copy` is required
// 6. Tuple structs and disjoint field borrowing

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn incr_x(&mut self) {
        self.0 += 1;
    }
}

fn main() {
    println!("=== 1. Reads and Writes with &self and &mut self ===");
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    // Calling &self method: Requires Read (R) permission
    println!("rect1 area (&self): {}", rect1.area());

    // Calling &mut self method: Requires Read (R) and Write (W) permission
    rect1.set_width(40);
    println!("rect1 after set_width (&mut self): {rect1:?}");

    // Reborrowing: An &mut reference can call an &self method via implicit reborrow (&*r)
    let rect_mut_ref: &mut Rectangle = &mut rect1;
    println!("area via &mut ref (implicit reborrow): {}", rect_mut_ref.area());

    println!("\n=== 2. Moves with self (By-Value) and Copy Semantics ===");
    // `max(self, other)` takes ownership (R + O).
    // Because Rectangle derives `Copy`, both rect1 and rect2 are bitwise copied on the stack,
    // leaving both original variables valid.
    let max_rect = rect1.max(rect2);
    println!("max_rect: {max_rect:?}");
    println!("rect1 still valid because Rectangle is Copy: {rect1:?}");

    println!("\n=== 3. set_to_max (*self behind &mut self) ===");
    // In `set_to_max`, `*self = self.max(other)` desugars to `Rectangle::max(*self, other)`.
    // If Rectangle did NOT implement Copy, this would fail with E0507 (cannot move out of *self).
    rect1.set_to_max(rect2);
    println!("rect1 after set_to_max: {rect1:?}");

    println!("\n=== 4. Tuple Structs & Method Mutation ===");
    let mut p = Point(0, 0);
    p.incr_x();
    println!("point after incr_x: ({}, {})", p.0, p.1);
}

impl Rectangle {
    // 1. Immutable borrow (&self): Requires Read (R)
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // 2. Mutable borrow (&mut self): Requires Read (R) + Write (W)
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // 3. By-value consumption (self): Requires Read (R) + Own (O)
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // 4. Moving out of `*self`: Allowed because Rectangle is Copy
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}
