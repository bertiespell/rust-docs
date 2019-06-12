/** 
The first task is to make minigrep accept its two command line arguments: the filename and a string to search for. That is, we want to be able to run our program with cargo run, a string to search for, and a path to a file to search in, like so:


$ cargo run searchstring example-filename.txt
*/
use std::env; // it’s conventional to bring the parent module into scope rather than the function

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
