use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::{Rc};

pub fn multiple_returns() -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<String>>>, Rc<RefCell<Option<bool>>>) {

    return (Rc::new(RefCell::new(Some(42))), Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(true))));
}

fn main() {
        // Ignoring return values
    println!("{}", "=== Ignoring return values ===".to_string());

        // Ignore all but first return value
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", (*num.borrow_mut().as_mut().unwrap()));

        // Ignore first and last return values
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", (*str.borrow_mut().as_mut().unwrap()));

        // Ignore first two return values
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", (*flag.borrow_mut().as_mut().unwrap()));

        // Ignoring in range loops
    println!("{}", "\n=== Ignoring in range loops ===".to_string());

    let mut slice = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40, 50])));

        // Ignore index, use only value
    println!("{}", "Values only:".to_string());
    for val in &(*slice.borrow_mut().as_mut().unwrap()) {
        print!("{} ", val);
    }
    println!();

        // Ignore value, use only index
    println!("{}", "Indices only:".to_string());
    for (i, _) in (*slice.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        print!("{} ", i);
    }
    println!();

        // Alternative: just use index (more idiomatic)
    println!("{}", "Indices (idiomatic):".to_string());
    for i in 0..(*slice.borrow_mut().as_mut().unwrap()).len() {
        print!("{} ", i);
    }
    println!();

        // Ignoring in map iteration
    println!("{}", "\n=== Ignoring in map iteration ===".to_string());

    let mut ages = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<i32>>>>::from([("Alice".to_string(), Rc::new(RefCell::new(Some(25)))), ("Bob".to_string(), Rc::new(RefCell::new(Some(30)))), ("Carol".to_string(), Rc::new(RefCell::new(Some(35))))]))));

        // Ignore values, use only keys
    println!("{}", "Keys only:".to_string());
    let mut keys = Rc::new(RefCell::new(Some(Vec::with_capacity((*ages.borrow().as_ref().unwrap()).len()))));
    for (name, _) in (*ages.borrow().as_ref().unwrap()).clone() {
        {(*keys.borrow_mut().as_mut().unwrap()).push(name); keys.clone()};
    }
    (*keys.borrow_mut().as_mut().unwrap()).sort();
    for name in &(*keys.borrow_mut().as_mut().unwrap()) {
        print!("{} ", name);
    }
    println!();

        // Ignore keys, use only values
    println!("{}", "Values only:".to_string());
    let mut values = Rc::new(RefCell::new(Some(Vec::with_capacity((*ages.borrow().as_ref().unwrap()).len()))));
    for (_, age) in (*ages.borrow().as_ref().unwrap()).clone() {
        {(*values.borrow_mut().as_mut().unwrap()).push((*age.borrow_mut().as_mut().unwrap())); values.clone()};
    }
    (*values.borrow_mut().as_mut().unwrap()).sort();
    for age in &(*values.borrow_mut().as_mut().unwrap()) {
        print!("{} ", age);
    }
    println!();

        // Using blank identifier in variable declarations
    println!("{}", "\n=== Blank identifier in declarations ===".to_string());

        // This would be useful for side effects only
    let _ = "This string is assigned but not used".to_string();

        // Multiple assignments with blank identifier
    let (mut a, _, mut c) = (Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(3))));
    print!("a={}, c={} (middle value ignored)\n", (*a.borrow_mut().as_mut().unwrap()), (*c.borrow_mut().as_mut().unwrap()));
}