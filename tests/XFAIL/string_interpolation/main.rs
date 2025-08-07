use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn main() {
    let mut name = Arc::new(Mutex::new(Some("World".to_string())));
    let mut age = Arc::new(Mutex::new(Some(25)));
    print!("Hello {}! You are {} years old.\n", (*name.lock().unwrap().as_mut().unwrap()), (*age.lock().unwrap().as_mut().unwrap()));
    let mut result = format!("Formatted: {}", format_slice(&Arc::new(Mutex::new(Some(vec![1, 2, 3])))));
    println!("{}", (*result.lock().unwrap().as_mut().unwrap()));
}