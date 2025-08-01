fn main() {
    let mut x: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut y: std::sync::Arc<std::sync::Mutex<Option<String>>> = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    let mut z: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(3.14)));
    let mut a = std::sync::Arc::new(std::sync::Mutex::new(Some(100)));
    let mut b = std::sync::Arc::new(std::sync::Mutex::new(Some("world".to_string())));
    let mut c = std::sync::Arc::new(std::sync::Mutex::new(Some(2.71)));
    println!("{} {} {} {}", "Variables:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()), (*z.lock().unwrap().as_mut().unwrap()));
    println!("{} {} {} {}", "Short vars:".to_string(), (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()));
}