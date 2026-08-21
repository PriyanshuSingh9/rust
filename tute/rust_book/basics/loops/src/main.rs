fn main() {
    // loop keyword tells Rust to execute a block of code over and over again either forever or until you
    // explicitly tell it to stop.
    let mut counter = 0;
    let result = loop {
        if counter < 5 {
            println!("again!");
        } else {
            break counter * 2;
            //  You can place the break keyword within the loop to tell the program when to stop executing the loop.
            // to pass the result of that operation out of the loop to the rest of your code we can use break with a value.
            // we can't use return here because it would exit the entire function, not just the loop.
            // Rust compiler treats a break expression and a return expression as having the value unit, or ()
        }
        counter += 1;
    };
    println!("result: {}", result);

    // If you have loops within loops, break and continue apply to the innermost loop at that point.
    // You can optionally specify a loop label on a loop that you can then use with break or continue to specify that
    // those keywords apply to the labeled loop instead of the innermost loop.
    let mut count = 0;
    // Loop labels must begin with a single quote.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // --- Streamlining Conditional Loops with while ---
    // A while loop evaluates a condition: while it is true, the code executes; otherwise, it exits.
    // This eliminates boilerplate nesting with loop, if, else, and break.
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // --- Looping Through a Collection with while (error-prone) ---
    // Using while to index into an array is error-prone (can panic if bounds/index check is wrong)
    // and **slower due to runtime bounds checks on each iteration**.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // --- Looping Through a Collection with for (concise & safe) ---
    // `for` loops eliminate out-of-bounds panics, off-by-one errors, and compile to more efficient machine code.
    for element in a {
        println!("the value is: {element}");
    }

    // --- Using for with Ranges and .rev() ---
    // A Range (start..end) generates sequential numbers ending before the end value.
    // .rev() reverses the range.
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
