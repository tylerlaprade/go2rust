use std::sync::{Arc, Mutex};

fn main() {
        // Basic nil comparisons
let mut p: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));
    if (*p.lock().unwrap()).is_none() {
        println!("{}", "p is nil".to_string());
    }

        // Assign nil
let mut q: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    if (*q.lock().unwrap()).is_none() {
        println!("{}", "q is nil".to_string());
    }

        // Non-nil pointer
let mut x = Arc::new(Mutex::new(Some(42)));
    { let new_val = (*x.lock().unwrap()).clone(); *p.lock().unwrap() = new_val; };
    if (*p.lock().unwrap()).is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    }

        // Set back to nil
*p.lock().unwrap() = None;
    if (*p.lock().unwrap()).is_none() {
        println!("{}", "p is nil again".to_string());
    }
}