// src/main.rs is always root
// When there is a cargo.toml file
// Another convention of Cargo’s is that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. The crate root files are passed by Cargo to rustc to actually build the library or binary.

// package can contain 0 or 1 library crates, and as many binary crates as you'd like
// There must be at least one crate (either a binary or a library) in a package

// If a package contains both src/main.rs and src/lib.rs, then it has two crates: a library and a binary, both with the same name. If we only had one of the two, the package would have either a single library or binary crate. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

// use -> a keyword that brings a path into scope
// pub -> a keyword to make items public
// as -> renaming items as you bring them into scope
// nested paths => these clean up large use lists

// modules let us organise code into groupings

mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                // you can also construct relative paths with super (Like in a file system - ..)
                super::super::voice::breathe_in(); // THIS DOES need to be public because it's in another module (note that the MODULE itself doesn't need to be public because we have that in scope)
                super::super::super::test(); // this doesn't either
                super::super::second_test(); // this doesn't need to be public
                third_test(); // neither does this
            }

            fn third_test() {

            }
        }
    }

    pub mod other {

    }

    mod voice { // all items are private by default
        pub fn breathe_in() {

        }
    }

    fn second_test() {

    }

    // You aren’t allowed to use private code defined in modules that are children of the current module. ??
    // You are allowed to use any code defined in ancestor modules or the current module.
}

fn test() {

}

use crate::sound::instrument; // similar to making a symlink in filesystem (Here instrument is the name of the module we can use)

// if you want to bring an item into scope and with RELATIVE PATH

use self::sound::other; // here we use self
use std::collections::HashMap;
use std::fmt;
use std::io; 

// we can solve the problem that fmt and io both import result by using as

use std::io::Result as IoResult;


fn main() {

    // absolute 
    crate::sound::instrument::woodwind::clarinet();
    // relateive
    sound::instrument::woodwind::clarinet();

    // now that they've been brought into scope above using use, we can actually just go :

    instrument::woodwind::clarinet();  // For functions, it’s considered idiomatic to specify the function’s parent module with use, and then specify the parent module when calling the function.  (so at its shortest - this should always be at least woodwind::clarinet())

    // For structs, enums, and other items, specifying the full path to the item with use is idiomatic. : 
    let mut map = HashMap::new();
    map.insert(1, 2);

    // The exception to this idiom is if the use statements would bring two items with the same name into scope, which isn’t allowed.

    // these two both are called result so we use namespacing
    fn function1() -> fmt::Result {
        unimplemented!();
    }

    // these two both are called result so we use namespacing
    fn function2() -> io::Result<()> {
        unimplemented!();
    }

    // uses as
    fn function3() -> IoResult<()> {
        unimplemented!();
    }

    let mut v = plant::Vegetable::new("Squash");

    v.name = String::from("Butternut Squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);

    let order1 = plant::Appetizer::Salad;
    let order2 = plant::Appetizer::Soup;

    // some of these ways of accessing modules are long and repetitive
    // let's use use ;)
    // this calls things into scope, and lets us use them as if they're local items

    test_mod();
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32, // this is private by default
    }

    pub enum Appetizer {
        Soup, // in contract, these are public by default
        Salad,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable { // this takes in a string slice
            Vegetable {
                name: String::from(name), //
                id: 1
            }
        }
    }
}

// when you import things they are PRIVATE
// you can use pub to RE-EXPORT things, so that calling code can use it

mod sound2 {
    pub mod instrument {
        pub fn clarinet() {

        }
    }
}

mod performance_group {
    pub use crate::sound2::instrument; // this let's use instrument from performance

    pub fn trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

pub fn test2() {
    performance_group::trio();
    performance_group::instrument::clarinet();
}

// TO USE EXTERNAL PACKAGES
// find what you want crates.io
// add to toml file
// then bring into scope using use!

// Use nested paths to improve imports

// instead of

use std::cmp::Ordering;
use std::marker;

// we can do
// use std::{cmp::Ordering, marker};

// Can also deduplicate paths

// use std::io;
// use std::io::Write; // this one is a complete path of the last

// instead do: 

// use std::io::{self, Write};

// bring everything in using glob operator!

use std::collections::*;

// You may want to organise code by moving modules into separate files

// e.g. move the mod sound to /sound.rs

mod another; // Using a semicolon after mod sound instead of a block tells Rust to load the contents of the module from another file with the same name as the module.

fn test_mod() {
    another::another2::test();
    another::test2(); // the another module is specified and NAMED by the filename itself (so there for things IN it, don't have to be wrapped in a mod {})

    // if they were to be wrapped we end up with line 202
    // another::another2::test(); we have ANOTHER level of wrapping here
    // extracting out into files is fine

}
