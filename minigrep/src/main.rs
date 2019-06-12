/** 
The first task is to make minigrep accept its two command line arguments: the filename and a string to search for. That is, we want to be able to run our program with cargo run, a string to search for, and a path to a file to search in, like so:


$ cargo run searchstring example-filename.txt
*/
use std::env; // it’s conventional to bring the parent module into scope rather than the function
use std::fs;
use std::io::prelude::*; // contains various useful traits for doing I/O, including file I/O.. In the same way that Rust has a general prelude that brings certain types and functions into scope automatically, the std::io module has its own prelude of common types and functions you’ll need when working with I/O. Unlike with the default prelude, we must explicitly add a use statement for the prelude from std::io.

fn main() {
    let args: Vec<String> = env::args().collect(); // the first value in the vector is "target/debug/minigrep", which is the name of our binary.. This matches the behavior of the arguments list in C
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
