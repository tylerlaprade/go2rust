use std::sync::{Arc, Mutex};

fn main() {
    let mut x = Arc::new(Mutex::new(Some(5)));
    { let new_val = (*x.lock().unwrap().as_mut().unwrap()) + 1; *x.lock().unwrap() = Some(new_val); };
    let mut p = x.clone();
    { let new_val = 10; *p.lock().unwrap() = Some(new_val); };

    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
}