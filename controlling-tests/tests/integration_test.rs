use adder;

#[test]
fn it_adds_twoooooo() {
    assert_eq!(4, adder::add_two(2));
}

// for test helper code (which we don't want to run)

// To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs. This is an alternate naming convention that Rust also understands

// Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

// After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module. Here’s an example of calling the setup function from the it_adds_two test in tests/integration_test.rs:

// instead do: 

use adder;

mod common; // import the common module

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
