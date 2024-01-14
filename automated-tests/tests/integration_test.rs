// have to include the project crate in the scope
use automated_tests;

// finds the common.rs file or a mod.rs file inside the common directory
mod common;

// no need for the #[cfg(test)] trait since cargo knows this is a test file
#[test]
fn it_adds_two() {
    // call the setup function which can be used by other test files
    common::setup();

    // we need to call the public api and not the inner adder
    assert_eq!(4, automated_tests::add_two(2));
}

// if you only want to run the integration tests
// cargo test --test integration_test

// we cannot directly test binary crates with integration tests
// common to see a binary crate as a thin wrapper around the library crate