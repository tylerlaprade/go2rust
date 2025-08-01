fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", "x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "p points to:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x after change:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
}