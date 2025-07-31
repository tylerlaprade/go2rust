fn main() {
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![64, 34, 25, 12, 22, 11, 90])));
    println!("{} {}", "Before:".to_string(), (*numbers.lock().unwrap().as_ref().unwrap()));
    (*sort.lock().unwrap().as_ref().unwrap()).ints(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))));
    println!("{} {}", "After:".to_string(), (*numbers.lock().unwrap().as_ref().unwrap()));
}