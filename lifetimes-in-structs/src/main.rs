// It is possible for structs to hold references
struct ImportantExcerpt<'a> {
    part: &'a str, // a string slice, which is a reference, and has a lifetime
}

// ****** This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field. *****

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    // i.e. if first_sentence became no longer valid, neither would Important Excerpt
}

// but this compiled despite both parameter and return value being references

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// because this used to have to always be written like this: 

fn first_word_previously<'a>(s: &'a str) -> &'a str {
    unimplemented!();
}

// but people found they were always writing the same life annotation signatures

// The deterministic patterns are called: 
// Lifetime elision rules

// The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. In this case, instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations that specify how the references relate to each other.

// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// The first rule is that each input lifetime (method or function parameter) that is a reference gets a lifetime specifier

// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32 (====> This is the reason why you don't always need to annotate lifetime types - when there is one parameter)

// The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

// LIFETIMES FOR STRUCTS and METHOD DEFINITIONS

// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // we're not required to annotate the lifetime of &self because of the third rule
        3
    }

    fn announce_and_return(&self, announcement: &str) -> &str { // this again doesn't need annotation because of the third rule
        // first rule gives &self and announcement lifetimes
        // but because one of the arguments is &self, the return type gets the lifetime of &self (and all the lifetimes have been accounted for)
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The Static Lifetime - 'static

// * Can live for the entire duration of the program
// * All string literals have the 'static lifetime: 
fn example_lifetime() {
    let s: &'static str = "I am a little static string literal";
}
// this text is stored in the programmes binary - which is always available
// You should always consider whether you're variable lasts the entire length of the programme before using static

// Mostly the problem on compilation is the result of a bug which could leave a dangling reference - solve that - rather than use static!