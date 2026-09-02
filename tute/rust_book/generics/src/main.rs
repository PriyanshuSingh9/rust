// The Rust Programming Language - Chapter 10.1: Generic Data Types
#![allow(unused)]

// When you recognize situations in your code with multiple struct or enum definitions that differ only
// in the types of the values they hold, you can avoid duplication by using generic types instead.
struct Point<T, U> {
    x: T,
    y: U,
}

// By declaring T and U as generic types after `impl`, Rust can identify that the types in the
// angle brackets in Point are generic types rather than concrete types.
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// We can also specify constraints on generic types when defining methods on the type.
// We could, for example, implement methods only on Point<f32, f32> instances rather than on
// Point<T, U> instances with any generic type:
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// IMPORTANT METHOD SPECIALIZATION RULES:
// 1. You cannot simultaneously implement specific and generic methods of the same name this way.
//    For example, if you implemented a general distance_from_origin for all types T and a specific
//    distance_from_origin for f32, then the compiler will reject your program: Rust does not know which
//    implementation to use when you call Point<f32>::distance_from_origin.
// 2. More generally, Rust does not have inheritance-like mechanisms for specializing methods as you might
//    find in an object-oriented language.
// 3. Additionally, unlike languages like Java where all objects have a set of core methods like
//    Object.toString(), there are no core methods in Rust.
// 4. Without restrictions, a generic type T has no capabilities: it cannot be printed, cloned, or mutated
//    (although it can be dropped).
//
// Example of unconstrained T failing:
// ```
// fn print_slice<T>(v: &[T]) {
//     for x in v {
//         println!("{x}"); // ERROR: cannot assume anything about T, including the ability to turn it into a string.
//     }
// }
// ```
// Therefore `println!("{x}")` is invalid because x: &T has no Display trait bound.

// Methods can also declare generic parameters that differ from the struct's generic parameters.
// Here X1 and Y1 are declared on the struct, while X2 and Y2 are declared on the method:
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Duplicate concrete functions for finding the largest item in a slice:
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic function:
// We declare a generic function that works with any type T in the signature so that the compiler knows what that name means.
// PartialOrd is a trait that tells the compiler that the type T can be compared (filtering out uncomparable objects like open files).
// Returning `&T` avoids moving out of `list` (E0507) and avoids requiring `T: Copy` or `T: Clone`.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // 1. Generic Functions
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&number_list);
    println!("The largest number via generic function is {result}");

    let result = largest(&char_list);
    println!("The largest char via generic function is {result}");

    // 2. Generic Structs
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both_integer: x={} y={}", both_integer.x, both_integer.y);
    println!("both_float: x={} y={}", both_float.x, both_float.y);
    println!(
        "integer_and_float: x={} y={}",
        integer_and_float.x, integer_and_float.y
    );

    // 3. Generic Methods & Concrete Specialization
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let float_p = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("float_p distance from origin: {}", float_p.distance_from_origin());

    // 4. Method-Level Generic Parameters (mixup)
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Performance Note:
// Using generic types will not make your program run any slower than it would with concrete types.
// Rust accomplishes this by performing monomorphization of the code using generics at compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types
// that are used when compiled.
