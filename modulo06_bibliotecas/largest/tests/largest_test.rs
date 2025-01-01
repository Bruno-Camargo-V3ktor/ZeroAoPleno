use largest::largest;

#[test]
fn test_largest() {
    let list = vec![1, 2, 3, 4, 5];
    assert_eq!(largest(&list), Some(5));
}
