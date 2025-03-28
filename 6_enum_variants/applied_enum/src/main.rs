
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn max_shape_area(shapes: Vec<Shape>) -> Option<Shape> {
    let mut max_shape = None;
    let mut max_area = 0.0;
    for shape in shapes {
        match shape {
            Shape::Circle(radius) => {
                let area = std::f64::consts::PI * radius * radius;
                println!("Circle area: {:.2}", area);
                if area > max_area {
                    max_area = area;
                    max_shape = Some(shape);
                }
            }
            Shape::Square(side) => {
                let area = side * side;
                println!("Square area: {:.2}", area);
                if area > max_area {
                    max_area = area;
                    max_shape = Some(shape);
                }
            }
            Shape::Triangle(base, height) => {
                let area = 0.5 * base * height;
                println!("Triangle area: {:.2}", area);
                if area > max_area {
                    max_area = area;
                    max_shape = Some(shape);
                }
            }
        }
    }
    max_shape
    
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 6.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();

    println!("Total area: {:.2} sq. units", total_area);

    let max_shape = max_shape_area(shapes);
    match max_shape {
        Some(shape) => println!("Maximum shape: {:?}", shape),
        None => println!("No shapes found."),
    }
}