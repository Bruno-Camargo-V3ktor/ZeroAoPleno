//Structs
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Impls
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}
