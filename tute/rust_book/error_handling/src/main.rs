// Chapter 9: Error Handling in Rust (TRPL Chapter 9)
// Covers:
// 9.1 Unrecoverable Errors with panic!
// 9.2 Recoverable Errors with Result, the `?` operator, and fs::read_to_string
// 9.3 To panic! or Not to panic!, Box<dyn Error> in main, and Custom Invariant Types
#![allow(dead_code, unused_variables)]

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    println!("==================================================");
    println!("       RUST BOOK CHAPTER 9: ERROR HANDLING        ");
    println!("==================================================");

    // 1. Recoverable Errors with Match & ErrorKind (9.2)
    demo_recoverable_match();

    // 2. Functional unwrap_or_else alternative (9.2)
    demo_unwrap_or_else();

    // 3. The `?` operator on Option (9.2)
    demo_question_mark_option();

    // 4. fs::read_to_string one-liner (9.2 last section)
    demo_fs_read_to_string();

    // 5. When humans know more than the compiler (9.3)
    demo_human_invariants();

    // 6. Custom validation types (9.3)
    demo_custom_types();

    // 7. Using `?` in main() returning Result<(), Box<dyn Error>> (9.2)
    // Attempting to open a non-existent file cleanly returns Err instead of panicking:
    println!("\n--- 7. main() Returning Result ---");
    let f = File::open("hello.txt");
    match f {
        Ok(_) => println!("hello.txt opened successfully"),
        Err(e) => println!("Returned Result from operation: {e}"),
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// 9.2: Recoverable Errors with Result & Match
// -----------------------------------------------------------------------------
fn demo_recoverable_match() {
    println!("\n--- 1. Handling Result with match & ErrorKind ---");
    let file_result = File::open("hello.txt");

    let greeting_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found; creating hello.txt...");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                }
            }
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    println!("File opened/created successfully.");
}

// -----------------------------------------------------------------------------
// 9.2: Functional Alternatives with Closures (unwrap_or_else)
// -----------------------------------------------------------------------------
fn demo_unwrap_or_else() {
    println!("\n--- 2. Functional Handling with unwrap_or_else ---");
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    println!("File handled cleanly with closures.");
}

// -----------------------------------------------------------------------------
// 9.2: Error Propagation with ? Operator
// -----------------------------------------------------------------------------
// Verbose propagation with match:
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let mut username_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Concise propagation with `?`:
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    // The `?` operator unwraps Ok or early-returns Err through From::from conversion
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 9.2 Last Section: The std::fs::read_to_string one-liner
fn read_username_fs_oneliner() -> Result<String, io::Error> {
    // Reads entire file into a String in a single standard library call
    fs::read_to_string("hello.txt")
}

fn demo_fs_read_to_string() {
    println!("\n--- 4. fs::read_to_string One-Liner ---");
    match read_username_fs_oneliner() {
        Ok(content) => println!("Read content: \"{content}\""),
        Err(e) => println!("Error reading file: {e}"),
    }
}

// -----------------------------------------------------------------------------
// 9.2: The `?` Operator on Option<T>
// -----------------------------------------------------------------------------
fn last_char_of_first_line(text: &str) -> Option<char> {
    // `?` on Option stops and returns None immediately if lines().next() is None
    text.lines().next()?.chars().last()
}

fn demo_question_mark_option() {
    println!("\n--- 3. Using ? on Option<T> ---");
    let text = "Hello world\nSecond line";
    println!("Last char of first line: {:?}", last_char_of_first_line(text));
    println!("Empty string test:       {:?}", last_char_of_first_line(""));
}

// -----------------------------------------------------------------------------
// 9.3: When Humans Know More Than the Compiler
// -----------------------------------------------------------------------------
fn demo_human_invariants() {
    println!("\n--- 5. Human Invariants vs. Compiler Verification ---");
    // Hardcoded IP address is mathematically known to be valid, so unwrap/expect is justified:
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address string must be valid");
    println!("Parsed loopback address: {home}");
}

// -----------------------------------------------------------------------------
// 9.3: Custom Types for Invariant Validation
// -----------------------------------------------------------------------------
// Encapsulates business logic to make out-of-bounds states impossible:
pub struct Guess {
    value: i32,
}

impl Guess {
    // Constructor validates invariants before instantiation:
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // Getter function provides read-only access (field `value` is private):
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn demo_custom_types() {
    println!("\n--- 6. Custom Validation Types (Guess Struct) ---");
    let valid_guess = Guess::new(42);
    println!("Constructed valid guess: {}", valid_guess.value());
    // Guess::new(200); // Would panic! with descriptive error message
}
