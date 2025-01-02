pub struct Point<X, Y> {
    pub x: X,
    pub y: Y,
}

impl<X, Y> Point<X, Y> {
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
