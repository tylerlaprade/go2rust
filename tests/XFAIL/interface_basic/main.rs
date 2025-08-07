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

pub fn print_any(v: Arc<Mutex<Option<Box<dyn Any>>>>) {
    println!("{} {}", "Value:".to_string(), (*v.lock().unwrap().as_mut().unwrap()));
}

fn main() {
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>>;

    { let new_val = 42; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is int:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());

    { let new_val = "hello".to_string(); *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is string:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());

    { let new_val = 3.14; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is float:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());

    let mut values = Arc::new(Mutex::new(Some(vec![1, "two".to_string(), 3.0])));
    println!("{} {}", "Mixed values:".to_string(), format_slice(&values));
}