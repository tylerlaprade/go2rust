use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::{Rc};

fn main() {
    let mut nums = Rc::new(RefCell::new(Some(vec![2, 3, 4])));
    let mut sum = Rc::new(RefCell::new(Some(0)));
    for num in &(*nums.borrow_mut().as_mut().unwrap()) {
        { let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    println!("{} {}", "sum:".to_string(), (*sum.borrow_mut().as_mut().unwrap()));

    for (i, num) in (*nums.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        if num == 3 {
        println!("{} {}", "index:".to_string(), i);
    }
    }

    let mut kvs = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<String>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some("apple".to_string())))), ("b".to_string(), Rc::new(RefCell::new(Some("banana".to_string()))))]))));
    for (k, v) in (*kvs.borrow().as_ref().unwrap()).clone() {
        print!("{} -> {}\n", k, (*v.borrow_mut().as_mut().unwrap()));
    }
}