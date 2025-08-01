fn main() {
    let mut builder: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>;
    (*builder.lock().unwrap().as_mut().unwrap()).write_string(std::sync::Arc::new(std::sync::Mutex::new(Some("Hello".to_string()))));
    (*builder.lock().unwrap().as_mut().unwrap()).write_string(std::sync::Arc::new(std::sync::Mutex::new(Some(" ".to_string()))));
    (*builder.lock().unwrap().as_mut().unwrap()).write_string(std::sync::Arc::new(std::sync::Mutex::new(Some("World".to_string()))));
    let mut result = (*builder.lock().unwrap().as_mut().unwrap()).string();
    println!("{}", (*result.lock().unwrap().as_mut().unwrap()));
}