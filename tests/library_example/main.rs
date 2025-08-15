mod math;
mod string;
use math::*;
use string::*;

use std::sync::{Arc, Mutex};

fn main() {
        // Test math functions
    let mut sum = add(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(3))));
    print!("5 + 3 = {}\n", (*sum.lock().unwrap().as_mut().unwrap()));

    let mut product = multiply(Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(7))));
    print!("4 * 7 = {}\n", (*product.lock().unwrap().as_mut().unwrap()));

        // Test string function
    let mut repeated = repeat(Arc::new(Mutex::new(Some("Go".to_string()))), Arc::new(Mutex::new(Some(3))));
    print!("Repeat(\"Go\", 3) = {}\n", (*repeated.lock().unwrap().as_mut().unwrap()));
}