fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut p = x.clone();
    let mut q = x.clone();
    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    { let new_val = 200; *q.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_ref().unwrap()));
}