fn main() {
    lets_use_our_trait();
}

// A trait - tells the Rust compiler about functionality a particular type has and can share with other types.
// SO: We can use traits to define shared behaviour
// Similar to, but not the same as, interfaces

// We want to make a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we need to request that summary by calling a summarize method on an instance.

pub trait Summary {
    fn summarize(&self) -> String; // the method signature, which describe the behaviours of the types that implement the trait
    // NOTE: ends with a semicolon - not an implementation

    // Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// they both implement the Summary trait!
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn lets_use_our_trait() {
    let tweet = Tweet {
        username: String::from("BertieSpell"),
        content: String::from("hello again"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}