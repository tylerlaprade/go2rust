fn main() {
    let mut name = std::sync::Arc::new(std::sync::Mutex::new(Some("World".to_string())));
    let mut age = std::sync::Arc::new(std::sync::Mutex::new(Some(25)));
    print!("Hello {}! You are {} years old.\n", (*name.lock().unwrap().as_ref().unwrap()), (*age.lock().unwrap().as_ref().unwrap()));
    let mut result = (*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("Formatted: %v".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3]))));
    println!("{}", (*result.lock().unwrap().as_ref().unwrap()));
}