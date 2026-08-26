// Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
// Crate Root: src/lib.rs defines the root module of this library crate (`restaurent`).
#![allow(dead_code, unused_variables, unused_imports)]

// 1. Multi-File Module Separation (Chapter 7.5)

// Declaring `mod front_of_house;` tells the compiler to look for `src/front_of_house.rs`.
mod front_of_house;

// 2. Re-Exporting with `pub use` (Chapter 7.4)
// Allows external code calling this library to access `hosting` directly via `restaurent::hosting`.
pub use crate::front_of_house::hosting;

// 3. Module Privacy Rules & Encapsulation (Chapter 7.3)
mod back_of_house {
    // Structs: Fields are private by default, even if the struct is marked `pub`.
    // If any field is private, a public constructor function is required.
    pub struct Breakfast {
        pub toast: String,      // Public field: caller can read and write
        seasonal_fruit: String, // Private field: only code inside `back_of_house` can access
    }

    impl Breakfast {
        // Public constructor needed because `seasonal_fruit` is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Enums: When an enum is marked `pub`, all of its variants are automatically public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // `super` refers to the parent module (the crate root in this case)
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// 4. Using Paths and `use` Bindings (Chapter 7.3 & 7.4)
pub fn eat_at_restaurant() {
    // Absolute path (starts with `crate::`)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (starts from current scope)
    front_of_house::hosting::add_to_waitlist();

    // Using re-exported or brought-into-scope item:
    hosting::add_to_waitlist();

    // Struct field access:
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // Allowed: `toast` is pub
    println!("I'd like {} toast please", meal.toast);

    // Private field access is rejected at compile-time:
    // meal.seasonal_fruit = String::from("blueberries"); // ERROR: field `seasonal_fruit` of struct `Breakfast` is private

    // Enum variant access:
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// 5. Idiomatic `use` Patterns & Disambiguation with `as`
// - For functions: Bring parent module into scope (`use crate::front_of_house::hosting; hosting::add_to_waitlist()`).
// - For structs/enums: Bring type directly into scope (`use std::collections::HashMap;`).
// - Disambiguation: Use `as` to rename items with identical names in the same scope:
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// 6. Nested Paths & The Glob Operator
// - Group common roots: `use std::{cmp::Ordering, io};`
// - Self import: `use std::io::{self, Write};`
// - Glob (imports all public items): `use std::collections::*;`
