// every reference in Rust has a lifetime

// most of the time this is implicit and inferred

// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

// Allows us to prevent dangling references
fn main() {
    let r;
    {
        let x = 5;
        r = &x // this won't compile - the value r is referring to has gone out of scope by the time we come to use it
    }
    println!("r: {}", r); // if this worked, r would be referencing memory that was deallocated when x went out of scope

    // RUST KNOWS!!   .... because of the Borrow Checker ;)
}
