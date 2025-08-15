use std::sync::{Arc, Mutex};

fn main() {
        // Basic variable declarations that currently fail
let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
    let mut y: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some("hello".to_string())));
    let mut z: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(3.14)));

        // Short variable declarations
let mut a = Arc::new(Mutex::new(Some(100)));
    let mut b = Arc::new(Mutex::new(Some("world".to_string())));
    let mut c = Arc::new(Mutex::new(Some(2.71)));

    println!("{} {} {} {}", "Variables:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()), (*z.lock().unwrap().as_mut().unwrap()));
    println!("{} {} {} {}", "Short vars:".to_string(), (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()));
}