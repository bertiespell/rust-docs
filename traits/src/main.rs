fn main() {
    lets_use_our_trait();
    lets_use_default_trait_impl();
}

// A trait - tells the Rust compiler about functionality a particular type has and can share with other types.
// SO: We can use traits to define shared behaviour
// Similar to, but not the same as, interfaces

// We want to make a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we need to request that summary by calling a summarize method on an instance.

pub trait Summary {
    fn summarize(&self) -> String; // the method signature, which describe the behaviours of the types that implement the trait
    // NOTE: ends with a semicolon - not an implementation

    // Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.

    // we can also have default implementations
    fn default_example(&self) -> String {
        String::from("Read more...")
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

//  Let’s say this lib.rs is for a crate we’ve called aggregator and someone else wants to use our crate’s functionality to implement the Summary trait on a struct defined within their library’s scope. They would need to bring the trait into their scope first. They would do so by specifying use aggregator::Summary;, which then would enable them to implement Summary for their type. The Summary trait would also need to be a public trait for another crate to implement it, which it is because we put the pub keyword before trait

// trait OR type must be local (we can't implement external traits on external types)

// This restriction is a property of programs called COHERENCE
// more specifically the ORPHAN RULE (because the parent type is not present)


// This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

fn lets_use_default_trait_impl() {
    let article = NewsArticle {
        headline: String::from("Headline here"),
        content: String::from("test"),
        location: String::from("somewhere"),
        author: String::from("me"),
    };

    println!("Example summary: {}", article.default_example());
}