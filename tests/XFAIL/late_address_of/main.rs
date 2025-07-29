fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(5)));
    x = std::sync::Arc::new(std::sync::Mutex::new(Some(x + 1)));
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(x.clone())));
    *p.lock().unwrap().as_ref().unwrap() = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    println!("{} {}", "x =".to_string(), x);
}