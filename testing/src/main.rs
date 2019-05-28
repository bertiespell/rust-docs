// At its simplest, a test in Rust is a function that’s annotated with the test attribute.

// Attributes are metadata about pieces of Rust code
// e.g. like Derive

// To change a function into a test function, add #[test] on the line before fn

// When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.
fn main() {
    println!("Hello, world!");
}
