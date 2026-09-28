trait Shape {
    fn area(&self) ->f64 ;
}

struct Circle(f64) ;

impl Shape for Circle {
    fn area(&self) ->f64 {
        std::f64::consts::PI * self.0 * self.0
    }
}

struct Square(f64) ;

impl Shape for Square {
    fn area(&self) ->f64 {
        self.0 * self.0
    }
}


fn main() {
    let v1: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle(10.)),
            Box::new(Square(20.)),
        ] ;

    for v in v1 {
        println!("Space: {}", v.area())
    } /* Out:
        Space: 314.1592653589793
        Space: 400
     */
}
