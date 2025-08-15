use std::sync::{Arc, Mutex};

trait Shape {
    fn area(&self) -> Arc<Mutex<Option<f64>>>;
    fn perimeter(&self) -> Arc<Mutex<Option<f64>>>;
}

#[derive(Debug)]
struct Rectangle {
    width: Arc<Mutex<Option<f64>>>,
    height: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Circle {
    radius: Arc<Mutex<Option<f64>>>,
}

impl Rectangle {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_ref().unwrap()) * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }

    pub fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2.0 * ((*self.width.clone().lock().unwrap().as_ref().unwrap()) + (*self.height.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some((*self.width.clone().lock().unwrap().as_ref().unwrap()) * (*self.height.clone().lock().unwrap().as_ref().unwrap()))));
    }
    fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2.0 * ((*self.width.clone().lock().unwrap().as_ref().unwrap()) + (*self.height.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl Circle {
    pub fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(3.14159 * (*self.radius.clone().lock().unwrap().as_ref().unwrap()) * (*self.radius.clone().lock().unwrap().as_ref().unwrap()))));
    }

    pub fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2 * 3.14159 * (*self.radius.clone().lock().unwrap().as_ref().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(3.14159 * (*self.radius.clone().lock().unwrap().as_ref().unwrap()) * (*self.radius.clone().lock().unwrap().as_ref().unwrap()))));
    }
    fn perimeter(&self) -> Arc<Mutex<Option<f64>>> {
        return Arc::new(Mutex::new(Some(2 * 3.14159 * (*self.radius.clone().lock().unwrap().as_ref().unwrap()))));
    }
}

pub fn print_shape_info(s: Arc<Mutex<Option<Box<dyn Shape>>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*(*s.lock().unwrap().as_mut().unwrap()).area().lock().unwrap().as_ref().unwrap()), (*(*s.lock().unwrap().as_mut().unwrap()).perimeter().lock().unwrap().as_ref().unwrap()));
}

fn main() {
    let mut rect = Rectangle { width: Arc::new(Mutex::new(Some(10))), height: Arc::new(Mutex::new(Some(5))) };
    let mut circle = Circle { radius: Arc::new(Mutex::new(Some(3))) };

    println!("{}", "Rectangle:".to_string());
    print_shape_info(rect.clone());

    println!("{}", "Circle:".to_string());
    print_shape_info(circle.clone());

        // Interface slice
    let mut shapes = Arc::new(Mutex::new(Some(vec![(*rect.lock().unwrap().as_mut().unwrap()), (*circle.lock().unwrap().as_mut().unwrap())])));
    println!("{}", "All shapes:".to_string());
    for (i, shape) in (*shapes.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Shape {}: ", i + 1);
        print_shape_info(Arc::new(Mutex::new(Some(shape))));
    }
}