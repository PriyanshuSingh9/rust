// The Rust Programming Language - Chapter 10.3: Validating References with Lifetimes
#![allow(unused)]

use std::fmt::Display;

// Lifetimes are another kind of generic that ensure that references are valid as long as we need them to be.
// We are only required to annotate types when multiple types are possible. In a similar way, we must annotate
// lifetimes when the lifetimes of references could be related in a few different ways.
// Lifetime annotations don’t change how long any of the references live.
// Rather, they describe the relationships of the lifetimes of multiple references to each other without
// affecting the lifetimes.
// Just as functions can accept any type when the signature specifies a generic type parameter, functions can
// accept references with any lifetime by specifying a generic lifetime parameter.

// Reference Syntax:
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetime Annotations in Struct Definitions:
// An instance of ImportantExcerpt cannot outlive the reference it holds in its `part` field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Annotations in Method Definitions:
// Lifetime parameter names for struct fields always need to be declared after the `impl` keyword
// and then used after the struct’s name.
impl<'a> ImportantExcerpt<'a> {
    // 1st elision rule applies: &self gets a lifetime, return value is an owned i32
    fn level(&self) -> i32 {
        3
    }

    // 3rd elision rule applies: because one of the parameters is &self, the return type
    // is assigned the lifetime of &self by default:
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    // 1. The 'static Lifetime
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference
    // can live for the entire duration of the program. All string literals have the 'static lifetime:
    let s: &'static str = "I have a static lifetime.";

    // 2. Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Demonstration of concrete lifetime scopes:
    // `result` is valid as long as both string1 and string2 are valid.
    {
        let string3 = String::from("longer string");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("The longest string in inner scope is {result2}");
    }

    // 3. Structs with Lifetimes
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence is {}", i.part);
    println!("Announced: {}", i.announce_and_return_part("Now boarding"));

    // 4. Putting It All Together: Generics, Trait Bounds, and Lifetimes
    let combined_result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "System update completed",
    );
    println!("Longest with announcement: {combined_result}");
}

// The generic lifetime `'a` in the signature indicates that the lifetime of the returned reference
// is the same as the smaller of the lifetimes of the references passed in (`x` and `y`).
// The return type needs a generic lifetime parameter because Rust can’t tell whether the reference being returned
// refers to x or y, because the if block returns x and the else block returns y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Summary: Generic Type Parameters, Trait Bounds, and Lifetimes Together
// This function declares both a lifetime parameter `'a` and a generic type parameter `T`
// with a trait bound `where T: Display`.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Elision Rules (Compiler Heuristics for Inferring Lifetimes):
// The compiler uses three rules to figure out what lifetimes references have when there aren't explicit annotations:
// 1. The compiler assigns a lifetime parameter to each parameter that’s a reference.
//    (e.g., `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`)
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
//    (e.g., `fn foo<'a>(x: &'a i32) -> &'a i32`)
// 3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of
//    `self` is assigned to all output lifetime parameters.
