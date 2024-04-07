use adder2;

mod common;

#[test]
fn it_adds_two() {
    let result = adder2::add(100, 101);
    common::setup();
    assert_eq!(result, 201);
}
