use std::sync::{Arc, Mutex};

fn main() {
    let mut x = Arc::new(Mutex::new(Some(10)));
    let mut y = x.clone();
    let mut z = Arc::new(Mutex::new(Some((*y.lock().unwrap().as_mut().unwrap()))));

    { let new_val = 20; *y.lock().unwrap() = Some(new_val); };
    { let new_val = 30; *z.lock().unwrap() = Some(new_val); };

    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
}