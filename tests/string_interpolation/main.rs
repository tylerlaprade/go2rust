use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn main() {
    let mut name = Rc::new(RefCell::new(Some("World".to_string())));
    let mut age = Rc::new(RefCell::new(Some(25)));
    print!("Hello {}! You are {} years old.\n", (*name.borrow_mut().as_mut().unwrap()), (*age.borrow_mut().as_mut().unwrap()));
    let mut result = Rc::new(RefCell::new(Some(format!("Formatted: {}", format_slice(&Rc::new(RefCell::new(Some(vec![1, 2, 3]))))))));
    println!("{}", (*result.borrow_mut().as_mut().unwrap()));
}