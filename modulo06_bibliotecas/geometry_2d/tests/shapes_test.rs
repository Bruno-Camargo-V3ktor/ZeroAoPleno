use geometry_2d::{point::Point, shapes::Rectangle};

#[test]
fn test_area_of_rectangle() {
    let rect = Rectangle::new(0.0, 10.0, 5.0, 5.0);

    assert_eq!(rect.area(), 25.0);
}

#[test]
fn test_perimeter_of_rectangle() {
    let rect = Rectangle::new(0.0, 10.0, 5.0, 5.0);

    assert_eq!(rect.perimeter(), 20.0);
}

#[test]
fn test_contains_points_inside_rectangle() {
    let rect = Rectangle::new(0.0, 10.0, 5.0, 5.0);
    let point = Point::new(5.0, 5.0);

    assert_eq!(rect.contains_point(&point), true);
}

#[test]
fn test_contains_points_outside_rectangle() {
    let rect = Rectangle::new(0.0, 10.0, 5.0, 5.0);
    let point = Point::new(6.0, 6.0);

    assert_eq!(rect.contains_point(&point), false);
}
