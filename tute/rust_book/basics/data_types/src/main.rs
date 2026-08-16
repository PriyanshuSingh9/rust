// Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so that it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.
// Rust is a statically typed language, which means that it must know the types of all variables at compile time.

fn main() {
    // A scalar type represents a single value.
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime.
    // compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics.
    // usize: unsigned integer sized to the platform's pointer size; commonly used for indexes and sizes.
    // isize: signed integer sized to the platform's pointer size; used when negative values may be needed.
    //
    // The floating-point default type is f64 because on modern CPUs, it's roughly the same speed as f32 but is capable of more precision.
    // All floating-point types are signed.

    // We specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks. Rust’s char type is 4 bytes in size and represents a Unicode scalar value,
    // which means it can represent a lot more than just ASCII.

    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}"); // 500, 6.4, 1

    // We can also access a tuple element directly by using a period (.) followed by the index of the value
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x: {x}, y: {y}, z: {z}"); // 500, 6.4, 1
    // This program creates the tuple x and then accesses each element of the tuple using their respective indices.
    // The tuple without any values has a special name, unit.
    // This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.
    let mut mut_tup: (i32, i32) = (1, 2);
    mut_tup.0 = 0;
    mut_tup.1 += 5;
    println!("mut_tup: {mut_tup:?}"); // (0, 7)

    // An array is another way to have a collection of multiple values.
    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    // Arrays are useful when you want data allocated on the stack rather than the heap,
    // or when you want to ensure that you always have a fixed number of elements.
    // (A vector is a similar collection provided by the standard library that is allowed to grow or shrink because its contents live on the heap).
    let a = [1, 2, 3, 4, 5];
    println!("a: {a:?}"); // [1, 2, 3, 4, 5]

    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December",
    ];
    println!("First month: {}", months[0]); // January

    // You write an array’s type using square brackets with the type of each element,
    // a semicolon, and then the number of elements in the array: [type; size]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {a:?}"); // [1, 2, 3, 4, 5]

    // You can also initialize an array to contain the same value for each element:
    // [value; length]
    let a = [3; 5]; // same as writing let a = [3, 3, 3, 3, 3];
    println!("a: {a:?}"); // [3, 3, 3, 3, 3]

    // Array Element Access
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
    // You can access elements of an array using indexing:
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}"); // first: 1, second: 2

    // Invalid Array Element Access
    // When you attempt to access an element using indexing, Rust checks that the index is less than the array length.
    // If the index is greater than or equal to the length, Rust will panic at runtime:
    // e.g., accessing a[10] results in "index out of bounds: the len is 5 but the index is 10"
    // This is an example of Rust’s memory safety principles: immediately exiting instead of allowing invalid memory access.
    // let element = a[10]; // This causes a panic / error: index out of bounds
}

