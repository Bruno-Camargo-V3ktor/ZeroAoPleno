use super::point::Point;

//Structs
pub struct Rectangle {
    pub point: Point,
    pub width: f64,
    pub height: f64,
}

// Impls
impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            point: Point::new(x, y),
            width,
            height,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.point.x
            && point.x <= (self.point.x + self.width)
            && point.y <= self.point.y
            && point.y >= (self.point.y - self.height)
    }

    pub fn print(&self) {
        println!(
            "Rectangle: point = ({}, {}), width = {}, height = {}",
            self.point.x, self.point.y, self.width, self.height
        )
    }
}
