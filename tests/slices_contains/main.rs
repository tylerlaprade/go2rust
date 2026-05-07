use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 3, 5, 7])));
    let mut words = Rc::new(RefCell::new(Some(vec!["red".to_string(), "green".to_string(), "blue".to_string()])));

    println!("{} {}", "has 3:".to_string(), (*Rc::new(RefCell::new(Some({ let __slice_holder = numbers.clone(); let __slice_guard = __slice_holder.borrow(); let __slice = __slice_guard.as_ref().unwrap(); let __value = 3; __slice.contains(&__value) }))).borrow().as_ref().unwrap()));
    println!("{} {}", "has 4:".to_string(), (*Rc::new(RefCell::new(Some({ let __slice_holder = numbers.clone(); let __slice_guard = __slice_holder.borrow(); let __slice = __slice_guard.as_ref().unwrap(); let __value = 4; __slice.contains(&__value) }))).borrow().as_ref().unwrap()));
    println!("{} {}", "has green:".to_string(), (*Rc::new(RefCell::new(Some({ let __slice_holder = words.clone(); let __slice_guard = __slice_holder.borrow(); let __slice = __slice_guard.as_ref().unwrap(); let __value = "green".to_string(); __slice.contains(&__value) }))).borrow().as_ref().unwrap()));
    println!("{} {}", "has yellow:".to_string(), (*Rc::new(RefCell::new(Some({ let __slice_holder = words.clone(); let __slice_guard = __slice_holder.borrow(); let __slice = __slice_guard.as_ref().unwrap(); let __value = "yellow".to_string(); __slice.contains(&__value) }))).borrow().as_ref().unwrap()));
    if (*Rc::new(RefCell::new(Some({ let __slice_holder = numbers.clone(); let __slice_guard = __slice_holder.borrow(); let __slice = __slice_guard.as_ref().unwrap(); let __value = 5; __slice.contains(&__value) }))).borrow().as_ref().unwrap()) {
        println!("{}", "condition number hit".to_string());
    }
    if !(*Rc::new(RefCell::new(Some({ let __slice_holder = words.clone(); let __slice_guard = __slice_holder.borrow(); let __slice = __slice_guard.as_ref().unwrap(); let __value = "yellow".to_string(); __slice.contains(&__value) }))).borrow().as_ref().unwrap()) {
        println!("{}", "condition word miss".to_string());
    }
}