use std::sync::{Arc, Mutex};

fn main() {
    let mut x = Arc::new(Mutex::new(Some(2)));
    match (*x.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "One".to_string());
        }
        2 => {
            println!("{}", "Two".to_string());
            // TODO: fallthrough not supported
        }
        3 => {
            println!("{}", "Three (via fallthrough)".to_string());
        }
        4 => {
            println!("{}", "Four".to_string());
        }
        _ => {
            println!("{}", "Other".to_string());
        }
    }

    println!("{}", "---".to_string());

    let mut grade = Arc::new(Mutex::new(Some(('B' as i32))));
    match (*grade.lock().unwrap().as_mut().unwrap()) {
        ('A' as i32) => {
            println!("{}", "Excellent!".to_string());
            // TODO: fallthrough not supported
        }
        ('B' as i32) => {
            println!("{}", "Good job!".to_string());
            // TODO: fallthrough not supported
        }
        ('C' as i32) => {
            println!("{}", "Passed".to_string());
        }
        ('D' as i32) => {
            println!("{}", "Barely passed".to_string());
        }
        ('F' as i32) => {
            println!("{}", "Failed".to_string());
        }
        _ => {}
    }

    println!("{}", "---".to_string());

    let mut n = Arc::new(Mutex::new(Some(15)));
    match true {
        true if (*n.lock().unwrap().as_mut().unwrap()) % 15 == 0 => {
            println!("{}", "FizzBuzz".to_string());
            // TODO: fallthrough not supported
        }
        true if (*n.lock().unwrap().as_mut().unwrap()) % 3 == 0 => {
            println!("{}", "Fizz".to_string());
        }
        true if (*n.lock().unwrap().as_mut().unwrap()) % 5 == 0 => {
            println!("{}", "Buzz".to_string());
        }
        _ => {
            println!("{}", (*n.lock().unwrap().as_mut().unwrap()));
        }
    }
}