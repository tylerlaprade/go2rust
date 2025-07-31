fn main() {
    (*rand.lock().unwrap().as_ref().unwrap()).seed(std::sync::Arc::new(std::sync::Mutex::new(Some((*time.lock().unwrap().as_ref().unwrap()).now()::unix_nano()))));
    println!("{} {}", "Random int:".to_string(), (*rand.lock().unwrap().as_ref().unwrap()).intn(std::sync::Arc::new(std::sync::Mutex::new(Some(100)))));
    println!("{} {}", "Random float:".to_string(), (*rand.lock().unwrap().as_ref().unwrap()).float64());
}