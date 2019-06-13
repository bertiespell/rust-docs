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

// EXAMPLE of making this public at the top level (rather than inside mingrep)
pub use minigrep::Config;

/**
 * ========== PUBLISHING ===========
 * 
 * 1) Make sure config is set (API token needed from crates.io)
 * 2) Make sure TOML file is accurate (must contain license) see docs: https://doc.rust-lang.org/1.29.0/book/2018-edition/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
 * 3) cargo publish
 * 
 * ========== VERSIONING ===========
 * 
 * Once a package is published it can't be deleted! (ensures consistency of builds)
 * Just change the version value (follow sem-ver) and republish
 * 
 * =========== YANKING =============
 * 
 * Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports yanking a crate version.
 * 
 * $ cargo yank --vers 1.0.1
 * 
 * Oops didn't mean to do that
 * 
 * $ cargo yank --vers 1.0.1 --undo 
 * 
 * A yank does not delete any code. For example, the yank feature is not intended for deleting accidentally uploaded secrets. If that happens, you must reset those secrets immediately.
 */

fn main() {
    // let args: Vec<String> = env::args().collect(); // the first value in the vector is "target/debug/minigrep", which is the name of our binary.. This matches the behavior of the arguments list in C    

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}