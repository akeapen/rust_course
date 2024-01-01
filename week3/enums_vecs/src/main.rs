#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

// takes a vector of shapes and returns the largest shape and area
fn largest_shape(shapes: &Vec<Shape>) -> (&Shape, f64) {
    let mut largest_area = 0.0;
    let mut largest_shape = &shapes[0];

    for shape in shapes {
        let area = match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        };
        println!("area: {:.2}", area);

        if area > largest_area {
            largest_area = area;
            largest_shape = shape;
        }
    }

    (largest_shape, largest_area)
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 5.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();

    println!("Total area: {:.2} sq. units", total_area);

    let (largest_shape, largest_area) = largest_shape(&shapes);
    println!(
        "The largest shape is a {:?} with an area of {:.2} sq. units",
        largest_shape, largest_area
    );
}
