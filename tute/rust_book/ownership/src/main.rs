// Ownership and Memory Semantics Demonstration
// The Rust Programming Language - Chapter 4.1

fn demo_compile_time_initialization() {
    println!("--- 1. Compile-Time Initialization Checks ---");
    let x = true;
    read(x);

    // Why Rust checks variable definition at compile-time, not runtime:
    // If we called `read(y)` before `let y = true;`, interpreted languages
    // would raise a NameError/ReferenceError at runtime on every variable access.
    // Rust performs definite assignment analysis at compile-time, eliminating
    // all runtime definition checks from the compiled machine code.
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn demo_box_and_move_semantics() {
    println!("\n--- 2. Box Allocation & Move Semantics ---");

    // Heap allocation: Box manages unique ownership of heap memory.
    // Stack: 8-byte pointer | Heap: 4 integers [10, 20, 30, 40]
    let a = Box::new([10, 20, 30, 40]);
    println!("a points to heap buffer: {:?}", a);

    // Destructive Move:
    // Rust performs a bitwise copy of the 8-byte stack pointer from `a` to `b`,
    // then statically invalidates `a` at compile-time.
    let b = a;
    println!("b now owns the heap buffer: {:?}", b);

    // Attempting to read `a` here is rejected at compile time with [E0382]:
    // println!("{:?}", a); // ERROR: borrow of moved value: `a`

    // Cloning: Deep Copy
    // Allocates a brand new buffer on the heap and copies all elements.
    let c = b.clone();
    println!("c is an independent clone on heap: {:?}", c);
    println!("b remains valid after clone: {:?}", b);
}

fn demo_dereferencing_vs_borrowing() {
    println!("\n--- 3. Dereferencing vs Borrowing ---");

    let a = Box::new([1, 2, 3, 4, 5]);

    // 1. Shared Reference to Box (&Box<T>):
    // Stack pointer pointing to `a` on the stack (double indirection).
    let ref_to_box = &a;
    println!("Reading through &Box: {}", ref_to_box[0]);

    // 2. Slice Reference (&[T]) via Deref Coercion:
    // Fat pointer (16 bytes: data pointer + length) directly into heap buffer.
    let slice_ref: &[i32] = &a[..];
    println!(
        "Reading through slice ref: len={}, first={}",
        slice_ref.len(),
        slice_ref[0]
    );

    // 3. Dereference to Stack (*a):
    // Copies the underlying array [i32; 5] from heap to callee stack frame.
    let stack_array: [i32; 5] = *a;
    println!("stack_array lives on stack: {:?}", stack_array);
    println!("a is still valid because [i32; 5] implements Copy: {:?}", a);
}

fn demo_string_heap_layout() {
    println!("\n--- 4. String Stack Descriptor & Function Boundaries ---");

    // String on stack: 24 bytes (ptr: 8B, len: 8B, capacity: 8B)
    let s1 = String::from("systems-rust");
    println!(
        "s1 initialized: '{s1}' (len={}, cap={})",
        s1.len(),
        s1.capacity()
    );

    // Moving ownership into function
    takes_ownership(s1);
    // println!("{s1}"); // ERROR: s1 was moved!

    // Copy semantics for scalar values
    let x = 42;
    makes_copy(x);
    println!("x remains valid after copy: {x}");

    // Returning ownership from function
    let s2 = gives_ownership();
    println!("s2 received ownership from function: '{s2}'");
}

fn takes_ownership(s: String) {
    println!("  takes_ownership received: '{s}' (dropped at end of function scope)");
} // s is dropped and freed here

fn makes_copy(x: i32) {
    println!("  makes_copy received: {x} (copied on stack)");
} // x exits scope, nothing to free

fn gives_ownership() -> String {
    let s = String::from("returned-from-callee");
    s // Ownership moved to caller
}

fn demo_heap_move_vs_stack_copy() {
    println!("\n--- 5. Heap Allocated (Move) vs Stack Only (Copy) Types ---");

    // HEAP-ALLOCATED TYPES (MOVE SEMANTICS -> SOURCE INVALIDATED):
    // List: String, Vec<T>, Box<T>, HashMap<K, V>, File, TcpStream
    let heap_string = String::from("heap-data");
    let moved_string = heap_string; // heap_string is now INVALID
    // println!("{heap_string}"); // ERROR: borrow of moved value

    let heap_vec = vec![1, 2, 3];
    let moved_vec = heap_vec; // heap_vec is now INVALID
    // println!("{:?}", heap_vec); // ERROR: borrow of moved value

    let heap_box = Box::new(99);
    let moved_box = heap_box; // heap_box is now INVALID
    // println!("{heap_box}"); // ERROR: borrow of moved value

    println!(
        "Heap-allocated items moved successfully: '{}', {:?}, {}",
        moved_string, moved_vec, moved_box
    );

    // STACK-ONLY PRIMITIVE TYPES (COPY TRAIT -> SOURCE REMAINS FULLY VALID):
    // List: i8..i128, u8..u128, f32, f64, bool, char, [T; N] (where T: Copy), tuples of Copy types
    let stack_int: i32 = 100;
    let copied_int = stack_int;

    let stack_bool: bool = true;
    let copied_bool = stack_bool;

    let stack_char: char = 'R';
    let copied_char = stack_char;

    let stack_array: [i32; 3] = [10, 20, 30];
    let copied_array = stack_array;

    let stack_tuple: (i32, bool) = (500, false);
    let copied_tuple = stack_tuple;

    println!(
        "Stack-only sources remain valid: int={}, bool={}, char={}, array={:?}, tuple={:?}",
        stack_int, stack_bool, stack_char, stack_array, stack_tuple
    );
    println!(
        "Copied targets: int={}, bool={}, char={}, array={:?}, tuple={:?}",
        copied_int, copied_bool, copied_char, copied_array, copied_tuple
    );
}

fn main() {
    demo_compile_time_initialization();
    demo_box_and_move_semantics();
    demo_dereferencing_vs_borrowing();
    demo_string_heap_layout();
    demo_heap_move_vs_stack_copy();
}
