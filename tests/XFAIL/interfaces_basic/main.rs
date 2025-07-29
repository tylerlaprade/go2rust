

#[derive(Debug)]
struct Rectangle {
    width: f64,
}

pub fn area() -> f64 {

    return r.width * r.height;
}

pub fn perimeter() -> f64 {

    return 2 * ;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

pub fn area() -> f64 {

    return 3.14159 * c.radius * c.radius;
}

pub fn perimeter() -> f64 {

    return 2 * 3.14159 * c.radius;
}

pub fn print_shape_info(s: Shape) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", s.area(), s.perimeter());
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    let mut circle = Circle { radius: 3 };
    println!("{}", "Rectangle:".to_string());
    print_shape_info(rect);
    println!("{}", "Circle:".to_string());
    print_shape_info(circle);
    let mut shapes = vec![rect, circle];
    println!("{}", "All shapes:".to_string());
    for (i, shape) in shapes.iter().enumerate() {
        print!("Shape {}: ", i + 1);
        print_shape_info(shape);
    }
}