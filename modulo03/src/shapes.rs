

// Traits
pub trait Shape {
    fn area( &self ) -> f64;
    fn perimeter( &self ) -> f64;
    fn draw( &self );
}

// Structs
pub struct Circle {
    pub radius: f64,
}

// Implements
impl Circle {
    pub fn new( radius: f64 ) -> Self { Self { radius } }
}

impl Shape for Circle {

    fn area( &self ) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn perimeter( &self ) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn draw( &self ) {
        println!( "Raio do circulo: {}", self.radius );
        println!( "Area: {}", self.area() );
        println!( "Perimito: {}", self.perimeter() );
    }

}
