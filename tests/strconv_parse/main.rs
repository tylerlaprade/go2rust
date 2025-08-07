use std::error::Error;
use std::sync::{Arc, Mutex};

fn main() {
    let mut str = Arc::new(Mutex::new(Some("42".to_string())));
    let (mut num, mut err) = match (*str.lock().unwrap().as_mut().unwrap()).parse::<i32>() { Ok(n) => (Arc::new(Mutex::new(Some(n))), Arc::new(Mutex::new(None))), Err(e) => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::new(e) as Box<dyn Error + Send + Sync>)))) };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    println!("{} {}", "Parsed number:".to_string(), (*num.lock().unwrap().as_mut().unwrap()));
}