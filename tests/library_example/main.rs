mod math;
mod string;
use math::*;
use string::*;

fn main() {
    let mut sum = std::sync::Arc::new(std::sync::Mutex::new(Some(add(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))))));
    print!("5 + 3 = {}\n", (*sum.lock().unwrap().as_ref().unwrap()));
    let mut product = std::sync::Arc::new(std::sync::Mutex::new(Some(multiply(std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(7)))))));
    print!("4 * 7 = {}\n", (*product.lock().unwrap().as_ref().unwrap()));
    let mut repeated = std::sync::Arc::new(std::sync::Mutex::new(Some(repeat(std::sync::Arc::new(std::sync::Mutex::new(Some("Go".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))))));
    print!("Repeat(\"Go\", 3) = {}\n", (*repeated.lock().unwrap().as_ref().unwrap()));
}