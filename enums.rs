//===============================Enums==========================

enum Shape {
    Rectangle(f64, f64), // width, height in float 64
    Circle(f64),         // radius height in float 64
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);
    let circle = Shape::Circle(1.0);
    calculate_area(circle);
}

fn calculate_area(shape: Shape) -> f64 {
    /*below matching the pattern from our enum i.e. shape and return whatever it is.

    we are not writing "return" keyword as it automatically knows the whole match shape block whatever it does that gonna be returned.*/
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
