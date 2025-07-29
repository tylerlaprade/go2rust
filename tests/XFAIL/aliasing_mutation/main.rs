fn main() {
    let mut x = 42;
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    let mut q = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    *p.lock().unwrap().as_ref().unwrap() = 100;
    *q.lock().unwrap().as_ref().unwrap() = 200;
    println!("{} {}", "x =".to_string(), x);
}