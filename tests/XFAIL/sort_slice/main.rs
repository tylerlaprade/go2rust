fn main() {
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![64, 34, 25, 12, 22, 11, 90])));
    println!("{} {}", "Before:".to_string(), (*numbers.lock().unwrap().as_mut().unwrap()));
    (*sort.lock().unwrap().as_mut().unwrap()).ints(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_mut().unwrap())))));
    println!("{} {}", "After:".to_string(), (*numbers.lock().unwrap().as_mut().unwrap()));
}