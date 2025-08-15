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
    let mut pattern = Arc::new(Mutex::new(Some(`\d+`.to_string())));
    let mut re = regexp.must_compile(Arc::new(Mutex::new(Some((*pattern.lock().unwrap().as_mut().unwrap())))));
    let mut text = Arc::new(Mutex::new(Some("I have 42 apples and 7 oranges".to_string())));
    let mut matches = (*re.lock().unwrap().as_mut().unwrap()).find_all_string(Arc::new(Mutex::new(Some((*text.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(-1))));
    println!("{} {}", "Numbers found:".to_string(), format_slice(&matches));
}