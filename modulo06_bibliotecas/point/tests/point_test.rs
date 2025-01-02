use point::Point;

#[test]
fn test_mixup() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point {
        x: "Hello",
        y: String::from("Ola"),
    };

    let mix = p1.mixup(p2);

    assert_eq!(mix.x, 5);
    assert_eq!(mix.y, String::from("Ola"));
}
