use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic fallthrough
    let mut x = Rc::new(RefCell::new(Some(2)));
    match (*x.borrow_mut().as_mut().unwrap()) {
        1 => {
            println!("{}", "One".to_string());
        }
        2 => {
            println!("{}", "Two".to_string());
            println!("{}", "Three (via fallthrough)".to_string());
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

        // Multiple fallthrough
    let mut grade = Rc::new(RefCell::new(Some(66)));
    match (*grade.borrow_mut().as_mut().unwrap()) {
        65 => {
            println!("{}", "Excellent!".to_string());
            println!("{}", "Good job!".to_string());
            println!("{}", "Passed".to_string());
        }
        66 => {
            println!("{}", "Good job!".to_string());
            println!("{}", "Passed".to_string());
        }
        67 => {
            println!("{}", "Passed".to_string());
        }
        68 => {
            println!("{}", "Barely passed".to_string());
        }
        70 => {
            println!("{}", "Failed".to_string());
        }
        _ => {}
    }

    println!("{}", "---".to_string());

        // Fallthrough with conditions
    let mut n = Rc::new(RefCell::new(Some(15)));
    match true {
        true if (*n.borrow_mut().as_mut().unwrap()) % 15 == 0 => {
            println!("{}", "FizzBuzz".to_string());
            println!("{}", "Fizz".to_string());
        }
        true if (*n.borrow_mut().as_mut().unwrap()) % 3 == 0 => {
            println!("{}", "Fizz".to_string());
        }
        true if (*n.borrow_mut().as_mut().unwrap()) % 5 == 0 => {
            println!("{}", "Buzz".to_string());
        }
        _ => {
            println!("{}", (*n.borrow_mut().as_mut().unwrap()));
        }
    }
}