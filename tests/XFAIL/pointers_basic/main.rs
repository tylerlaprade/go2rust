#[derive(Debug)]
struct Point {
    x: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    y: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", "Value of x:".to_string(), (*x.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Address of x:".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Value through pointer:".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Modified x:".to_string(), (*x.lock().unwrap().as_ref().unwrap()));
    let mut point = Point { x: 10, y: 20 }.clone();
    println!("{} {}", "Point:".to_string(), (*point.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Point X:".to_string(), (*point.lock().unwrap().as_ref().unwrap()).x);
    println!("{} {}", "Point Y:".to_string(), (*point.lock().unwrap().as_ref().unwrap()).y);
    { let new_val = 30; *(*point.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 40; *(*point.lock().unwrap().as_ref().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Modified point:".to_string(), (*point.lock().unwrap().as_ref().unwrap()));
    let mut q = std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()))));
    { let new_val = 200; *q.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x after modifying through q:".to_string(), (*x.lock().unwrap().as_ref().unwrap()));
    let mut newPoint = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc<std::sync::Mutex<Option<Point>>>::default())));
    { let new_val = 5; *(*newPoint.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 15; *(*newPoint.lock().unwrap().as_ref().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {}", "New point:".to_string(), (*newPoint.lock().unwrap().as_ref().unwrap()));
}