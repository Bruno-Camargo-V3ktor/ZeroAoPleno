use repositorio::notify;

#[test]
fn test_notify() {
    let msg1 = "Hello World";
    let msg2 = String::from("Rust is awesome");

    assert_eq!(notify(&msg1), ());
    assert_eq!(notify(&msg2), ());
}
