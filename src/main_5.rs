// enums with values 
enum Shape {
    Circle (f64), // variant associated with data (radius)
    Square (f64), // (side length)
    Rectangle (f64, f64), // (width, height)
}

// function to calculate area based on shape 
fn calculate_area (shape : Shape) -> f64 {
    // first need to find out which shape it is 
    // we do this using "PATTERN MATCHING"
    let area = match shape {
        // if shape is Rectangle - we return a * b 
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(s) => s * s,
    };

    return area;
}

fn main (){
    // create instances of different shapes 
    let circle = Shape::Circle(4.5);
    println!("{}", calculate_area(circle));

    let rectangle = Shape::Rectangle(4.5, 8.7);
    println!("{}", calculate_area(rectangle));

    let square = Shape::Square(4.5);
    println!("{}", calculate_area(square));
}


