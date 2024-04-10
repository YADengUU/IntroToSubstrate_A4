// for nice precision, instead of integers, the floats can be used in this part
//  and we set the trait bound to f64
trait CalculableBound {
    fn area(&self) -> f64;
}

// the different shapes
struct Circle {
    radius: f64,
}

impl CalculableBound for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius //pi*radius^2
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl CalculableBound for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height // (b*h)/2
    }
}

struct Square {
    side: f64,
}

impl CalculableBound for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

struct Rectangle {
    width1: f64,
    width2: f64,
}

impl CalculableBound for Rectangle {
    fn area(&self) -> f64 {
        self.width1 * self.width2
    }
}

// function to show the result
fn print_area<T: CalculableBound>(shape: T) {
    println!("Area: {}", shape.area());
}

// for the four example shapes
fn main() {
    let circle = Circle { radius: 1.5 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let rectangle = Rectangle { width1: 1.5, width2: 2.5 };
    let square = Square { side: 5.5 };

    print_area(circle);
    print_area(triangle);
    print_area(rectangle);
    print_area(square);
}
