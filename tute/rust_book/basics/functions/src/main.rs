// Rust doesn't care where you define your functions, only that they're defined somewhere
// in a scope that can be seen by the caller (before or after main).

fn main() {
    println!("Hello, world!");
    another_function();

    // Parameters are special variables that are part of a function's signature.
    // Concrete values passed in are called arguments.
    print_number(5); // 5 is an argument
    print_labeled_measurement(5, 'h');

    // Statements and Expressions:
    // Rust is an expression-based language.
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.

    // `let y = 6;` is a statement.
    // Statements do not return values, so you cannot assign a let statement to another variable:
    // let x = (let y = 6); // ERROR: expected expression, found `let` statement

    // A new scope block created with curly brackets is an expression:
    let y = {
        let x = 3;
        x + 1 // No semicolon: expressions do not end with semicolons
    };
    println!("The value of y is: {y}"); // 4

    // Functions with Return Values:
    // We declare the return type after an arrow (->).
    // The return value of the function is synonymous with the value of the final expression
    // in the body block. You can also use `return` keyword for early returns.
    let x = five();
    println!("The value of x is: {x}"); // 5

    let result = plus_one(5);
    println!("The value of result is: {result}"); // 6

    // Blocks are expressions and can be passed directly as arguments:
    let z = plus_one({
        let a = 1;
        a + 1
    });
    println!("The value of z is: {z}"); // 3
}

fn another_function() {
    println!("Another function.");
}

// In function signatures, you MUST declare the type of each parameter.
// This design decision allows the compiler to almost never need type annotations elsewhere
// to infer types, and gives helpful error messages.
fn print_number(x: i32) {
    println!("The value of x is: {x}");
}

// When defining multiple parameters, separate parameter declarations with commas:
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions can return values without let statements or macros;
// a solitary expression returns its value:
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // If you add a semicolon to the end of an expression (e.g. `x + 1;`),
    // you turn it into a statement, which returns () (unit type), causing a mismatched types error [E0308].
    x + 1
}
