fn main() {
    let mut x = 42;
    let mut y = x + 1;
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    *p.lock().unwrap().as_ref().unwrap() = 100;
    println!("{} {}", "x =".to_string(), x);
    println!("{} {}", "y =".to_string(), y);
}