// The Rust Programming Language - Chapter 10.2: Traits (Defining Shared Behavior)
#![allow(unused)]

use traits::{notify, returns_summarizable, NewsArticle, Pair, SocialPost, Summary};

fn main() {
    // 1. Implementing Traits on Structs & Calling Methods
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
    println!("{}", post.read_more());
    println!("{}", post.summarize_author());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("\nArticle summary: {}", article.summarize());
    println!("{}", article.summarize_author());

    // 2. Traits as Parameters (`notify`)
    println!("\n--- Traits as Parameters ---");
    notify(&post);
    notify(&article);

    // 3. Returning Types That Implement Traits
    println!("\n--- Returning impl Trait ---");
    let summarizable = returns_summarizable();
    println!("Returned item: {}", summarizable.summarize());

    // 4. Conditionally Implementing Methods via Trait Bounds
    println!("\n--- Conditional Method Implementation ---");
    let pair = Pair::new(10, 20);
    pair.cmp_display();
}
