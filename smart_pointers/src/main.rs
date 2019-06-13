/**
 * =========== Boxes ===========
 * 
 * Allow you to store data on the heap (the box is a pointer)
 * 
 * Used when: 
 * 
 * 1) You have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
 * 
 * 2) When you have a large amount of data and you want to transfer ownership, but want to ensure data won't be copied when you do
 * 
 * 3) When you want to own a value and you only care that it's a type that implements a specific trait (rather than being of a specific type) (trait object)
 */
fn main() {
    println!("Hello, world!");
}
