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