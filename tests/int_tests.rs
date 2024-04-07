use adder2;

#[test]
fn it_adds_two() {
    let result = adder2::add(100, 101);
    assert_eq!(result, 201);
}
