fn main() {
    let mut now = (*time.lock().unwrap().as_mut().unwrap()).now();
    println!("{} {}", "Current time:".to_string(), (*now.lock().unwrap().as_mut().unwrap()));
    let mut future = (*now.lock().unwrap().as_mut().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(24 * (*time.lock().unwrap().as_mut().unwrap()).hour))));
    println!("{} {}", "Tomorrow:".to_string(), (*future.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Unix timestamp:".to_string(), (*(*now.lock().unwrap().as_mut().unwrap()).unix().lock().unwrap().as_mut().unwrap()));
}