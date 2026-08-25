// structs give you a way of grouping together related fields and data
//  enums give you a way of saying a value is one of a possible set of values
#![allow(unused)]
enum Message {
    Quit,
    // you can attach information to a variant and a rust will create a function for initialisation
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // With if, the condition needs to evaluate to a Boolean value, but here it can be any type.
        match self {
            Message::Quit => println!("Quit"),
            // extracting x and y from the move variant
            Message::Move { x, y } => println!("Move: x={x}, y={y}"),
            Message::Write(s) => println!("Write: {s}"),
            Message::ChangeColor(r, g, b) => println!("ChangeColor: r={r}, g={g}, b={b}"),
            // this is a catch all pattern used to handle any other variant not matched above
            // use a variable name to attach values instead of discarding them(_)
            _ => {}
        }
    }
}
fn main() {
    let msg = Message::Move { x: 7, y: 18 };
    msg.call();

    // Option enum encodes the very common scenario in which a value could be something, or it could be nothing.
    // Rust doesn't have the null feature
    // Since, Option<T> uses generic types it can wrap any rust type

    let some_number = Some(5); // type: Opption<i32>
    let some_char = Some('e'); // type: Option<char>

    // As each options type is different due to generic parameters we have to annotate explicitly
    let absent_number: Option<i32> = None;

    // using a match block where we attach teh variants to varibles like s in the above example we move the
    // ownership from heap stored variables thus for such varibles we match on references
    let opt = Some(String::from("Hello"));
    match &opt {
        Some(s) => println!("{s}"),
        None => {}
    }
    // &opt is interpreted by the compiler as Option<&String>
    // Rust will “push down” the reference from the outer enum, &Option<String>, to the inner field,

    // `if let` matches a single pattern and ignores all others, eliminating `_ => ()` boilerplate
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // `if let ... else` handles one specific variant vs all other variants:
    let msg_write = Message::Write(String::from("Hello with if let"));
    if let Message::Write(text) = &msg_write {
        println!("Found a write message: {text}");
    } else {
        println!("Not a write message");
    }
}
