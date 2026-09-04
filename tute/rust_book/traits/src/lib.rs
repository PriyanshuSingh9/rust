#![allow(unused)]

use std::fmt::{Debug, Display};

// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to
// accomplish some purpose.

// Coherence and the Orphan Rule:
// We can’t implement external traits on external types. For example, we can’t implement the Display trait on
// Vec<T> within our crate, because Display and Vec<T> are both defined in the standard library and
// aren’t local to our crate. This restriction is part of a property called coherence, and more
// specifically the orphan rule, so named because the parent type is not present.
// This rule ensures that other people’s code can’t break your code and vice versa.
// Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which
// implementation to use.
pub trait Summary {
    fn summarize(&self) -> String;

    // Default implementations in traits: we can override them for our own types
    fn read_more(&self) -> String {
        String::from("Read more...")
    }

    // Default implementations can call other methods in the same trait, even if those other methods don’t have
    // a default implementation. In this way, a trait can provide a lot of useful functionality and only require
    // implementors to specify a small part of it.
    fn author(&self) -> String;

    fn summarize_author(&self) -> String {
        format!("Read more from {}", self.author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn author(&self) -> String {
        self.username.clone()
    }
}

// Traits as Parameters:
// Define a notify function that calls the summarize method on its item parameter,
// which is of some type that implements the Summary trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The `impl Trait` syntax works for straightforward cases but is syntactic sugar for a longer form known as
// a trait bound; it looks like this:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// Multiple trait bounds using the `+` syntax:
// pub fn notify_display(item: &(impl Summary + Display));
// pub fn notify_display<T: Summary + Display>(item: &T);

// Clearer Trait Bounds with where Clauses:
// Functions with too many trait bounds become hard to read; we use the where clause to clean up the syntax.
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// Returning Types That Implement Traits:
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}
// Note: You can only use `impl Trait` if you’re returning a single concrete type. For example, code that returns either a NewsArticle
// or a SocialPost based on a boolean condition with the return type specified as `impl Summary` will NOT compile.
// (Returning disparate trait implementors requires trait objects with dynamic dispatch, covered in Chapter 17).

// Using Trait Bounds to Conditionally Implement Methods:
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally
// for types that implement the specified traits.
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket Implementations:
// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are
// used extensively in the Rust standard library (e.g. `impl<T: Display> ToString for T`).
