fn main() {
    // rust variables are immutable by default
    // mut keyword allows you to make a variable mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants are always immutable and must be type annotated on declaration
    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
    // The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
    let y = 7;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");
    // let mut spaces="    ";
    // spaces=spaces.len(); this throws a type mismatch error
}
