use adder;

#[test]
fn it_adds_twoooooo() {
    assert_eq!(4, adder::add_two(2));
}

// for test helper code (which we don't want to run)

// To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs. This is an alternate naming convention that Rust also understa