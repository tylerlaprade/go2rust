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
    println!("{} {}", "Program name:".to_string(), (*os.lock().unwrap().as_mut().unwrap())::args[0]);
    println!("{} {}", "Arguments:".to_string(), format_slice(&(*os.lock().unwrap().as_mut().unwrap())::args[1..].to_vec()));
    println!("{} {}", "Total args:".to_string(), (*os.lock().unwrap().as_mut().unwrap())::args.len());
}