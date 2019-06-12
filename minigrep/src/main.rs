/** 
The first task is to make minigrep accept its two command line arguments: the filename and a string to search for. That is, we want to be able to run our program with cargo run, a string to search for, and a path to a file to search in, like so:


$ cargo run searchstring example-filename.txt
*/
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
extern crate minigrep;
use std::process;
use std::env; // it’s conventional to bring the parent module into scope rather than the function
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // the first value in the vector is "target/debug/minigrep", which is the name of our binary.. This matches the behavior of the arguments list in C    

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}