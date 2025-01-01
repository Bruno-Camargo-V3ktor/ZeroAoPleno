use maior_str::Par;

#[test]
fn test_par() {
    let x = 5;
    let y = 10;

    let par = Par::novo(&x, &y);
    assert_eq!(par.primeiro(), &x);
    assert_eq!(par.segundo(), &y);
}
