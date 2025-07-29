fn main() {
    let mut x = 42;
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    println!("{} {}", "x:".to_string(), x);
    println!("{} {}", "p points to:".to_string(), *p.lock().unwrap().as_ref().unwrap());
    *p.lock().unwrap().as_ref().unwrap() = 100;
    println!("{} {}", "x after change:".to_string(), x);
}