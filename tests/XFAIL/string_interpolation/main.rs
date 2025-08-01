fn main() {
    let mut name = std::sync::Arc::new(std::sync::Mutex::new(Some("World".to_string())));
    let mut age = std::sync::Arc::new(std::sync::Mutex::new(Some(25)));
    print!("Hello {}! You are {} years old.\n", (*name.lock().unwrap().as_mut().unwrap()), (*age.lock().unwrap().as_mut().unwrap()));
    let mut result = format!("Formatted: {}", vec![1, 2, 3]);
    println!("{}", (*result.lock().unwrap().as_mut().unwrap()));
}