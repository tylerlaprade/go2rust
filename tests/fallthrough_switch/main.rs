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

        // Multiple fallthrough
    let mut grade = Rc::new(RefCell::new(Some(('B' as i32))));
    match (*grade.borrow_mut().as_mut().unwrap()) {
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

        // Fallthrough with conditions
    let mut n = Rc::new(RefCell::new(Some(15)));
    match true {
        true if (*n.borrow_mut().as_mut().unwrap()) % 15 == 0 => {
            println!("{}", "FizzBuzz".to_string());
            // TODO: fallthrough not supported
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