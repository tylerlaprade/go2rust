use std::sync::{Arc, Mutex};

fn main() {
    let mut now = time.now();
    println!("{} {}", "Current time:".to_string(), (*now.lock().unwrap().as_mut().unwrap()));

    let mut future = now.add(Arc::new(Mutex::new(Some(24 * (*(*time.lock().unwrap().as_mut().unwrap())::hour.lock().unwrap().as_ref().unwrap())))));
    println!("{} {}", "Tomorrow:".to_string(), (*future.lock().unwrap().as_mut().unwrap()));

    println!("{} {}", "Unix timestamp:".to_string(), (*now.unix().lock().unwrap().as_ref().unwrap()));
}