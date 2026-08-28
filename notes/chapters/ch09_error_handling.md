# Chapter 9: Error Handling in Rust (Unrecoverable Panics vs. Recoverable Results)

This notebook provides a complete first-principles analysis of Chapter 9 of the Rust Book, covering panic stack unwinding, `Result<T, E>` error propagation mechanics, the `?` operator and `From` trait coercions, `main` return signatures, and custom invariant validation types.

---

## 1. Error Handling Philosophy: Recoverable vs. Unrecoverable

Rust separates errors into two distinct categories based on their operational severity:

```text
               ┌────────────────────────────────────────────────────────┐
               │                     ERROR HANDLING                     │
               └───────────────┬────────────────────────┬───────────────┘
                               │                        │
                               ▼                        ▼
               ┌────────────────────────┐   ┌────────────────────────┐
               │  RECOVERABLE ERRORS    │   │  UNRECOVERABLE ERRORS  │
               ├────────────────────────┤   ├────────────────────────┤
               │ • Expected conditions  │   │ • Bug in program logic │
               │   (file not found,     │   │ • Invariant violation  │
               │    network timeout)    │   │ • Out-of-bounds index  │
               │ • Type: Result<T, E>   │   │ • Macro: panic!()      │
               │ • Caller decides how   │   │ • Unwinds or aborts    │
               │   to recover / retry   │   │   the process          │
               └────────────────────────┘   └────────────────────────┘
```

---

## 2. Unrecoverable Errors with `panic!`

When `panic!` executes, by default Rust starts **stack unwinding**:
1. It walks back up the call stack, cleaning up (dropping) data in each stack frame.
2. It prints the failure message, file location, and line number to `stderr`.
3. It exits the thread with a non-zero exit code.

### 2.1 Unwinding vs. Aborting
In embedded or safety-critical environments where binary size and cleanup overhead must be minimized, you can configure Rust to **abort immediately** on panic without unwinding:

In `Cargo.toml`:
```toml
[profile.release]
panic = 'abort'
```

### 2.2 Enabling Backtraces
Stack backtraces require setting the `RUST_BACKTRACE` environment variable:
- `RUST_BACKTRACE=1 cargo run`: Prints filtered backtrace pointing to project source lines.
- `RUST_BACKTRACE=full cargo run`: Prints all stack frames including standard library internals.

---

## 3. Recoverable Errors with `Result<T, E>`

The `Result` enum represents success (`Ok`) or failure (`Err`):

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 3.1 Matching on `ErrorKind`
```rust
use std::fs::File;
use std::io::ErrorKind;

let file = match File::open("hello.txt") {
    Ok(f) => f,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating file: {e:?}"),
        },
        other_error => panic!("Problem opening file: {other_error:?}"),
    },
};
```

### 3.2 Functional Combinators (`unwrap_or_else`)
Idiomatic Rust replaces nested `match` statements with functional closures:

```rust
let file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating file: {error:?}");
        })
    } else {
        panic!("Problem opening file: {error:?}");
    }
});
```

### 3.3 Shortcuts for Panicking: `unwrap` vs. `expect`
- `.unwrap()`: Returns value inside `Ok`, or calls `panic!` with default message.
- `.expect("msg")`: Returns value inside `Ok`, or calls `panic!` with the specified custom message. **Always prefer `expect` over `unwrap`** in production to give context to failure points.

---

## 4. Propagating Errors with the `?` Operator

Instead of handling errors locally, functions can propagate errors to the caller using `?`.

### 4.1 How `?` Operates (The Asymmetry Invariant)
- On **`Err(e)`**: It early-returns `Err(From::from(e))` out of the enclosing function immediately.
- On **`Ok(val)`**: It unwraps `val` and continues execution in the local function body.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // `?` unwraps Ok or early-returns Err
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username) // Required because `username` is `String`, but return type is `Result`
}
```

### 4.2 The `From` Trait Automatic Conversion
When `?` encounters an error, it calls `From::from(error)` to convert the incoming error type into the error type declared in the calling function's return signature. This allows unifying disparate subsystem errors (e.g. `io::Error`, `serde_json::Error`, `parse::ParseIntError`) into a single domain error enum.

### 4.3 Standard Library One-Liner
```rust
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // Produces Result<String, io::Error> directly
}
```

### 4.4 The `?` Operator on `Option<T>`
The `?` operator also works on `Option<T>`: returns `None` early on `None`, or unwraps `Some(val)`:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

### 4.5 Using `?` in `main()`
`main` can return `Result<(), Box<dyn Error>>` to propagate errors directly to the runtime:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

---

## 5. To `panic!` or Not to `panic!` (Guidelines)

| Situation | Recommended Choice | Rationale |
| :--- | :--- | :--- |
| **Examples, Prototypes, Tests** | `panic!` / `unwrap` / `expect` | Clearer focus on example logic without error plumbing. |
| **Human Knowledge > Compiler** | `expect("known invariant")` | When you mathematically know an operation cannot fail (e.g. `"127.0.0.1".parse()`). |
| **Expected Failure (I/O, parsing, network)** | `Result<T, E>` | Caller can retry, fallback, or report a user-friendly error. |
| **Contract / Invariant Violation** | `panic!` | Caller passed invalid parameters violating API preconditions (memory safety at risk). |

---

## 6. Custom Types for Invariant Validation

Instead of repeatedly checking bounds across function boundaries, encode validation into dedicated types:

```rust
pub struct Guess {
    value: i32, // Private field enforces encapsulation
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
Any function taking `Guess` as an argument is guaranteed that `1 <= value <= 100` without performing runtime assertions.
