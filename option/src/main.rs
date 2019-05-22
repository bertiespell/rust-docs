fn main() {
    println!("Hello, world!");
}

// Option - another enum defined by the standard library
// Encodes the idea that something could be SOMETHING or NOTHING
// Expressing this in the type system checks that we are handling errors properly
// Prevents bugs

// Rust does not have NULL (a value which means there is no value)

// However the concept still needs to be conveyed:
// The value either doesn't exist, or is absent or invalid for some reason

// INSTEAD

// Rust has an enum (Option<T>) which encodes that a value can be PRESENT or ABSENT

enum Option<T> {
    Some(T),
    None,
}