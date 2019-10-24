use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// Call the module common
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
