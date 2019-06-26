fn main() {
    println!("Hello, world!");
}


/**
Features of Advanced Lifetimes

1. Lifetime subtyping: ensures that one lifetime outlives another lifetime
2. Lifetime bounds: specifies a lifetime for a reference to a generic type
3. Inference of trait object lifetimes: allows the compiler to infer trait object lifetimes and when they need to be specified
4. The anonymous lifetime: making elision more obvious
 */

// 1. Lifetime Subtyping

// Lifetime subtyping specifies that one lifetime should outlive another lifetime. To explore lifetime subtyping, imagine we want to write a parser. We’ll use a structure called Context that holds a reference to the string we’re parsing. We’ll write a parser that will parse this string and return success or failure. The parser will need to borrow the Context to do the parsing.

// For simplicity’s sake, the parse function returns Result<(), &str>. That is, the function will do nothing on success and, on failure, will return the part of the string slice that didn’t parse correctly. A real implementation would provide more error information and would return a structured data type when parsing succeeds. We won’t be discussing those details because they aren’t relevant to the lifetimes part of this example.

// To keep this code simple, we won’t write any parsing logic. However, it’s very likely that somewhere in the parsing logic we would handle invalid input by returning an error that references the part of the input that is invalid; this reference is what makes the code example interesting in regard to lifetimes. Let’s pretend that the logic of our parser is that the input is invalid after the first byte. Note that this code might panic if the first byte is not on a valid character boundary; again, we’re simplifying the example to focus on the lifetimes involved.

struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>
}

impl <'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}