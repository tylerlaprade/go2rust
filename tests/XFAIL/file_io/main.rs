fn main() {
    let (mut file, mut err) = (*os.lock().unwrap().as_mut().unwrap()).create(std::sync::Arc::new(std::sync::Mutex::new(Some("test.txt".to_string()))));
    if (*err.lock().unwrap().as_mut().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*file.lock().unwrap().as_mut().unwrap()).close() // TODO: defer not yet supported
    (*file.lock().unwrap().as_mut().unwrap()).write_string(std::sync::Arc::new(std::sync::Mutex::new(Some("Hello, World!".to_string()))));
    println!("{}", "File written successfully".to_string());
}