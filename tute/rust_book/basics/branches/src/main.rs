fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // is not reached as the condition is satisified before the control flow reaches here
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if number {
    //     println!("Does not compile")
    // }
    // this block does not compile as the condition must be a boolean truthy and falsy values don't exist in rust

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // let number = if condition { 5 } else { "six" }; Does not compile
    // the values that have the potential to be results from each arm of the if must be the same type
}
