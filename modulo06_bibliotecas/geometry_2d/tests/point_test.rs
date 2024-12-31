use geometry_2d::point::Point;

#[test]
fn test_distance_between_points() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);

    assert_eq!(p1.distance(&p2), 5.0);
}

#[test]
fn test_move_point_to_new_position() {
    let mut p1 = Point::new(0.0, 0.0);
    p1.move_to(3.0, 3.0);

    assert_eq!((p1.x, p1.y), (3.0, 3.0));
}
