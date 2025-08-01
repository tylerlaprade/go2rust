fn main() {
    let mut str = std::sync::Arc::new(std::sync::Mutex::new(Some("42".to_string())));
    let (mut num, mut err) = match (*str.lock().unwrap().as_mut().unwrap()).parse::<i32>() { Ok(n) => (std::sync::Arc::new(std::sync::Mutex::new(Some(n))), std::sync::Arc::new(std::sync::Mutex::new(None))), Err(e) => (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))) };
    if (*err.lock().unwrap().as_mut().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    println!("{} {}", "Parsed number:".to_string(), (*num.lock().unwrap().as_mut().unwrap()));
}