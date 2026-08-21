fn main() {
    // --- Array Slicing ---
    // A slice ([T]) is a *view* into contiguous memory: a "fat pointer" holding (data pointer, length).
    // Slicing is zero-copy: &v[1..4] points into the original buffer rather than duplicating values.
    let v = [10, 20, 30, 40, 50];

    // Range syntax: start..end (end exclusive), ..n (from start), n.. (through last), .. (everything).
    // These work identically on arrays and Vecs because both coerce to &[T];
    // range slicing is really an operation on slices.
    let full = &v[..];
    let mid = &v[1..4];
    let first_two = &v[..2];
    let rest = &v[2..];
    println!(
        "full: {full:?}, mid: {mid:?}, first_two: {first_two:?}, rest: {rest:?}"
    );

    // Bare `let x = v[1..4];` does NOT compile:
    //   error[E0277]: the size for values of type `[i32]` cannot be known at compile time
    // Rust must know every variable's stack size up front, but a slice's length
    // is only known at runtime. So a bare slice can only exist behind a pointer:
    // &[T], &mut [T], Box<[T]> - each is a fat (ptr, len) pair with a known size.
    //
    //   [i32; 5]  -> array            -> size known
    //   [i32]     -> slice            -> size unknown (cannot be a local)
    //   &[i32]    -> slice reference  -> size known

    // --- Independent Copies (not views) ---
    // A borrowed view writes through to the original. When you need duplicate
    // data that you own outright:

    // Copy into an owned Vec (heap allocates and copies elements; requires T: Clone):
    let mut owned_mid = v[1..4].to_vec();
    println!("owned_mid: {owned_mid:?}");
    owned_mid[0] = 99;
    println!("after mutation -> owned_mid: {owned_mid:?}, v untouched: {v:?}");

    // Copy into an existing fixed-size buffer without allocating.
    // Lengths must match exactly or this panics.
    let mut trio = [0i32; 3];
    trio.copy_from_slice(&v[1..4]);
    println!("trio: {trio:?}");

    // Build a new array by hand (only practical when indices are compile-time known):
    let manual = [v[1], v[2], v[3]];
    println!("manual: {manual:?}");

    // Indexing/slicing is bounds-checked at runtime: v[0..10] here would panic.
    // Rust makes copying explicit instead of copying on every slice, otherwise an
    // expression like huge_array[100..900] would silently allocate/copy.

    // --- Slices as Function Parameters ---
    // Prefer &[T] params: accepts arrays, Vecs, and sub-slices alike.
    println!("sum of mid: {}", sum(&v[1..4]));

    // Choose return types based on what the caller needs:
    let borrowed = borrow_section(&v);
    let owned = own_section(&v);
    println!("borrowed: {borrowed:?} (still tied to v), owned: {owned:?} (independent)");

    // Rule of thumb: pass/return borrows (&[T]) by default; use to_vec()/clone()
    // only when the caller needs data that outlives the input.
}

fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// Borrowed: zero-copy, but the result cannot outlive the input.
fn borrow_section(nums: &[i32]) -> &[i32] {
    &nums[1..4]
}

// Owned: caller gets independent data it can keep and mutate freely.
fn own_section(nums: &[i32]) -> Vec<i32> {
    nums[1..4].to_vec()
}
