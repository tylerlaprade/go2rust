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
    println!("{} {}", "Program name:".to_string(), (*(*os.lock().unwrap().as_mut().unwrap())::args.lock().unwrap().as_ref().unwrap())[0]);
    println!("{} {}", "Arguments:".to_string(), format_slice(&Arc::new(Mutex::new(Some((*(*os.lock().unwrap().as_mut().unwrap())::args.lock().unwrap().as_ref().unwrap())[1..].to_vec())))));
    println!("{} {}", "Total args:".to_string(), (*(*os.lock().unwrap().as_mut().unwrap())::args.lock().unwrap().as_ref().unwrap()).len());
}