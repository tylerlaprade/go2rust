fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut y = std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) + 1)));
    let mut p = x.clone();
    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "y =".to_string(), (*y.lock().unwrap().as_mut().unwrap()));
}