fn main() {
    let mut now = (*time.lock().unwrap().as_ref().unwrap()).now();
    println!("{} {}", "Current time:".to_string(), (*now.lock().unwrap().as_ref().unwrap()));
    let mut future = (*now.lock().unwrap().as_ref().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(24 * (*time.lock().unwrap().as_ref().unwrap()).hour))));
    println!("{} {}", "Tomorrow:".to_string(), (*future.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Unix timestamp:".to_string(), (*now.lock().unwrap().as_ref().unwrap()).unix());
}