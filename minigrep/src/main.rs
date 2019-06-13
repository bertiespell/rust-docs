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

// WORKSPACES => Only have top level lock file. Declare workspaces in top level TOML. Making all crates in the workspace use the same dependencies means the crates in the workspace will always be compatible with each other. 

/**
 * ======= BINARIES VS LIBRARIES ========
 * 
 * The cargo install command allows you to install and use binary crates locally. This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on crates.io. Note that you can only install packages that have binary targets. A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary, as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs. Usually, crates have information in the README file about whether a crate is a library, has a binary target, or both.
 * 
 * All binaries installed with cargo install are stored in the installation root’s bin folder. If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be $HOME/.cargo/bin. Ensure that directory is in your $PATH to be able to run programs you’ve installed with cargo install.
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