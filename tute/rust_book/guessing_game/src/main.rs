use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!!");

    // game continues till the user guesses the correct number
    loop {
        println!("Please input your guess:");

        // variables are immutable by default
        // :: syntax in the ::new line indicates that new is an associated function of the String type
        let mut guess = String::new();
        let secret = rand::rng().random_range(0..=100);
        /*
        stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the
        standard input for your terminal.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /*
         The full job of read_line is to take whatever the user types into standard input and append that into a string
         (without overwriting its contents), so we therefore pass that string as an argument.
         The string argument needs to be mutable so that the method can change the string’s content.
         The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code
         access one piece of data without needing to copy that data into memory multiple times.
         references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess
        // variable name rather than forcing us to create two unique variables

        println!("You guessed: {guess}");

        // Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes
        // that are possible when you compare two values.
        // cmp method compares two values and can be called on anything that can be compared( similar data types ).
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Large"),
        }
        // A match expression  is made up of arms. An arm consists of a pattern to match against, and the code that
        // should be run if the value given to match fits that arm’s pattern.
        // Lets you express a variety of situations your code might encounter, and they make sure you handle them all.

        // println!("The secret number was: {secret}");
    }
}
