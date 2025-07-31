fn main() {
    let mut name = (*flag.lock().unwrap().as_ref().unwrap()).string(std::sync::Arc::new(std::sync::Mutex::new(Some("name".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("World".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("a name to say hello to".to_string()))));
    (*flag.lock().unwrap().as_ref().unwrap()).parse();
    print!("Hello {}!\n", (*name.lock().unwrap().as_ref().unwrap()));
}