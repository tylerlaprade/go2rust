

#[derive(Debug)]
struct Rectangle {
    width: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Circle {
    radius: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

impl Rectangle {
    pub fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.width * self.height)));
    }

    pub fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(2 * )));
    }
}

impl Circle {
    pub fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(3.14159 * self.radius * self.radius)));
    }

    pub fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(2 * 3.14159 * self.radius)));
    }
}

pub fn print_shape_info(s: std::sync::Arc<std::sync::Mutex<Option<Shape>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*s.lock().unwrap().as_ref().unwrap()).area(), (*s.lock().unwrap().as_ref().unwrap()).perimeter());
}

fn main() {
    let mut rect = std::sync::Arc::new(std::sync::Mutex::new(Some(Rectangle { width: 10, height: 5 })));
    let mut circle = std::sync::Arc::new(std::sync::Mutex::new(Some(Circle { radius: 3 })));
    println!("{}", "Rectangle:".to_string());
    print_shape_info(std::sync::Arc::new(std::sync::Mutex::new(Some((*rect.lock().unwrap().as_ref().unwrap())))));
    println!("{}", "Circle:".to_string());
    print_shape_info(std::sync::Arc::new(std::sync::Mutex::new(Some((*circle.lock().unwrap().as_ref().unwrap())))));
    let mut shapes = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![(*rect.lock().unwrap().as_ref().unwrap()), (*circle.lock().unwrap().as_ref().unwrap())])));
    println!("{}", "All shapes:".to_string());
    for (i, shape) in (*shapes.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("Shape {}: ", i + 1);
        print_shape_info(std::sync::Arc::new(std::sync::Mutex::new(Some(shape))));
    }
}