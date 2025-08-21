use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct Point {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}

fn main() {
        // Basic pointer operations
    let mut x = Arc::new(Mutex::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", "Value of x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Address of x:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Value through pointer:".to_string(), (*p.lock().unwrap().as_ref().unwrap()));

        // Modify through pointer
    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Modified x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

        // Pointer to struct
    let mut point = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(10))), y: Arc::new(Mutex::new(Some(20))) })));
    println!("{} {}", "Point:".to_string(), (*point.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Point X:".to_string(), (*(*point.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Point Y:".to_string(), (*(*point.lock().unwrap().as_ref().unwrap()).y.lock().unwrap().as_ref().unwrap()));

        // Modify struct through pointer
    { let new_val = 30; *(*point.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 40; *(*point.lock().unwrap().as_mut().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Modified point:".to_string(), (*point.lock().unwrap().as_mut().unwrap()));

        // Pointer aliasing
    let mut q = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))));
    { let new_val = 200; *q.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x after modifying through q:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

        // New pointer allocation
    let mut newPoint = Arc::new(Mutex::new(Some(Arc<Mutex<Option<Point>>>::default())));
    { let new_val = 5; *(*newPoint.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 15; *(*newPoint.lock().unwrap().as_mut().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {}", "New point:".to_string(), (*newPoint.lock().unwrap().as_mut().unwrap()));
}