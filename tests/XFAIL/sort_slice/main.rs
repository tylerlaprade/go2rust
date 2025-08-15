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
    let mut numbers = Arc::new(Mutex::new(Some(vec![64, 34, 25, 12, 22, 11, 90])));
    println!("{} {}", "Before:".to_string(), format_slice(&numbers));
    sort.ints(Arc::new(Mutex::new(Some((*numbers.lock().unwrap().as_mut().unwrap())))));
    println!("{} {}", "After:".to_string(), format_slice(&numbers));
}