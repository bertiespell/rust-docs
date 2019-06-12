/** 
The first task is to make minigrep accept its two command line arguments: the filename and a string to search for. That is, we want to be able to run our program with cargo run, a string to search for, and a path to a file to search in, like so:


$ cargo run searchstring example-filename.txt
*/
use std::env; // it’s conventional to bring the parent module into scope rather than the function
use std::fs;
use std::io::prelude::*; // contains various useful traits for doing I/O, including file I/O.. In the same way that Rust has a general prelude that brings certain types and functions into scope automatically, the std::io module has its own prelude of common types and functions you’ll need when working with I/O. Unlike with the default prelude, we must explicitly add a use statement for the prelude from std::io.

/**
Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.

As long as your command line parsing logic is small, it can remain in main.rs.

When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

Calling the command line parsing logic with the argument values
Setting up any other configuration
Calling a run function in lib.rs
Handling the error if run returns an error
 */
fn main() {
    let args: Vec<String> = env::args().collect(); // the first value in the vector is "target/debug/minigrep", which is the name of our binary.. This matches the behavior of the arguments list in C
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let config = parse_config(&args);

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone(); // There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of its runtime cost.
    Config { query, filename }
    // (query, filename) // we could put the two values into one struct and give each of the struct fields a meaningful name. Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.
}