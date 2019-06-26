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

// Now we get to the point of this section: the Rust feature lifetime subtyping specifies that one lifetime parameter lives at least as long as another one. In the angle brackets where we declare lifetime parameters, we can declare a lifetime 'a as usual and declare a lifetime 'b that lives at least as long as 'a by declaring 'b using the syntax 'b: 'a.
struct Context<'s>(&'s str); // This string could have any other lifetime! Not necessarily the same as Parser

struct Parser<'a, 's: 'a> { // the syntax here 's: 'a => this means that 's MUST LIVE at least as long as 'a!!!... Now the reference to Context in the Parser and the reference to the string slice in the Context have different lifetimes; we’ve ensured that the lifetime of the string slice is longer than the reference to the Context.
    context: &'a Context<'s> // here the lifetime of Context's str is now not the same.. this time we used different parameters depending on whether the reference goes with the string slice or with Context
}

impl <'a, 's> Parser<'a, 's> {
    // Let's take another look at what is happening here...
    fn parse(&self) -> Result<(), &'s str> { // Remember the elision rules? If we annotate the lifetimes of the references rather than eliding, the signature would be as follows:
        // fn parse<'a>(&'a self) -> Result<(), &'a str> {
        Err(&self.context.0[1..]) // The parse_context function can’t see that within the parse function, the string slice returned will outlive Context and Parser and that the reference parse_context returns refers to the string slice, not to Context or Parser.
    }
}

// We’ll add a function that takes an instance of Context, uses a Parser to parse that context, and returns what parse returns. This code doesn’t quite work.
fn parse_context(context: Context) -> Result<(), &str> { // Here the str returned needs to live as long as context (which doesn't go out of scope at the end of the function - because it will be in the block that CALLED this function (and passed it as a variable - also this funciton does not consume context (importantly!) - so the reference can live on))
    Parser { context: &context }.parse() // These errors state that the Parser instance that is created and the context parameter live only until the end of the parse_context function. But they both need to live for the entire lifetime of the function.

    // In other words, Parser and context need to outlive the entire function and be valid before the function starts as well as after it ends for all the references in this code to always be valid. The Parser we’re creating and the context parameter go out of scope at the end of the function, because parse_context takes ownership of context.
}

// 2. ~~~~~~~~~~ Lifetime Bounds on References To Generic Types ~~~~~~~~~~~

/**

In the “Trait Bounds” section in Chapter 10, we discussed using trait bounds on generic types. We can also add lifetime parameters as constraints on generic types; these are called lifetime bounds. Lifetime bounds help Rust verify that references in generic types won’t outlive the data they’re referencing.

 */

// Without explicitly constraining the lifetime 'a in relation to the generic parameter T, Rust will error because it doesn’t know how long the generic type T will live:
// NOTE: As with the example above - I think there must be some changes to the language because this no longer errors in the way the documentation says...
struct Ref<'a, T>(&'a T);

// If we need to - we can specifiy lifetime bounds in a similar way to the above (with advanced subtyping for lifetimes)
struct Ref<'a, T: 'a>(&'a T);
