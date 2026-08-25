// Automatically implements the std::fmt::Debug trait for Rectangle.
// This allows printing the struct using debug formatting ({:?} or {:#?}).
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! prints expression, line number, and returns ownership of the evaluated value
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    // Calling an associated function (constructor) using `::` namespace syntax
    let mut sq = Rectangle::square(40);

    // Pass a reference (&rect1) to dbg! to avoid moving ownership of rect1
    dbg!(&rect1);

    // {:?} and {:#?} use the derived Debug trait ({:#?} produces pretty multiline output)
    println!("rectangle is: {rect1:#?}");

    // Method call syntax using dot notation (Rust applies automatic referencing here)
    println!("area is: {}", rect1.area());

    // Method taking another instance as a borrowed parameter
    println!(
        "rect1 {rect1:?} can hold rect2 {rect2:?}: {}",
        rect1.can_hold(&rect2)
    );

    println!("square is: {sq:#?}");

    // Calling a method that takes `&mut self` to mutate struct fields in place
    sq.set_width(100);
    println!("square is: {sq:#?}");
}

// Chapter 5.3: Method Syntax
// `impl` block defines methods and associated functions namespaced to `Rectangle`.
// Note: Types can also have multiple separate `impl` blocks.
impl Rectangle {
    // Method: First parameter is `self: &Self` (shorthand: `&self`).
    // Borrows the instance immutably for read-only access.
    // `Self` is an alias for the type in the `impl` block (`Rectangle`).
    fn area(self: &Self) -> u32 {
        self.height * self.width
    }

    // Method with multiple parameters: borrows `self` and borrows `other: &Rectangle`.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function: Does NOT take `self` as a parameter.
    // Often used as constructor functions (e.g. `String::from`).
    // Called using namespace syntax: `Rectangle::square(40)`.
    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }

    // Method with `&mut self`: Borrows the instance mutably to alter its fields.
    // Requires the instance variable (`sq`) to be declared as mutable (`let mut`).
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

