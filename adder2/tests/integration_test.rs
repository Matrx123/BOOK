use adder2::add2;

#[test]
fn check_add2() {
    let result = add2(3);
    assert_eq!(result, 5);
}

#[test]
fn check_add2_return() -> Result<(), String> {
    if 2 + 2 == 5 { Ok(()) } else { Err(String::from("Else part Son!")) }
}
