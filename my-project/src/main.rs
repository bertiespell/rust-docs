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
fn main() {

    // absolute 
    crate::sound::instrument::woodwind::clarinet();
    // relateive
    sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("Squash");

    v.name = String::from("Butternut Squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);

    let order1 = plant::Appetizer::Salad;
    let order2 = plant::Appetizer::Soup;
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