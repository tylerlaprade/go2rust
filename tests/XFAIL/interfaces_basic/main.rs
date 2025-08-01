trait Shape {
    fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>>;
    fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>>;
}

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
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }

    pub fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(2.0 * ((*self.width.clone().lock().unwrap().as_mut().unwrap()) + (*self.height.clone().lock().unwrap().as_mut().unwrap())))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*self.width.clone().lock().unwrap().as_mut().unwrap()) * (*self.height.clone().lock().unwrap().as_mut().unwrap()))));
    }
    fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(2.0 * ((*self.width.clone().lock().unwrap().as_mut().unwrap()) + (*self.height.clone().lock().unwrap().as_mut().unwrap())))));
    }
}

impl Circle {
    pub fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }

    pub fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*2.0 * 3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*(*3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()).lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
    fn perimeter(&self) -> std::sync::Arc<std::sync::Mutex<Option<f64>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*2.0 * 3.14159.lock().unwrap().as_mut().unwrap()) * (*self.radius.clone().lock().unwrap().as_mut().unwrap()))));
    }
}

pub fn print_shape_info(s: std::sync::Arc<std::sync::Mutex<Option<Box<dyn Shape>>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*s.lock().unwrap().as_mut().unwrap()).area(), (*s.lock().unwrap().as_mut().unwrap()).perimeter());
}

fn main() {
    let mut rect = Rectangle { width: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), height: std::sync::Arc::new(std::sync::Mutex::new(Some(5))) };
    let mut circle = Circle { radius: std::sync::Arc::new(std::sync::Mutex::new(Some(3))) };
    println!("{}", "Rectangle:".to_string());
    print_shape_info(rect.clone());
    println!("{}", "Circle:".to_string());
    print_shape_info(circle.clone());
    let mut shapes = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![(*rect.lock().unwrap().as_mut().unwrap()), (*circle.lock().unwrap().as_mut().unwrap())])));
    println!("{}", "All shapes:".to_string());
    for (i, shape) in (*shapes.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Shape {}: ", i + 1);
        print_shape_info(std::sync::Arc::new(std::sync::Mutex::new(Some(shape))));
    }
}