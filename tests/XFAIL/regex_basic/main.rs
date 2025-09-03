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
    let mut pattern = Rc::new(RefCell::new(Some(`\d+`.to_string())));
    let mut re = (*regexp.borrow_mut().as_mut().unwrap())::must_compile(Rc::new(RefCell::new(Some((*pattern.borrow_mut().as_mut().unwrap())))));
    let mut text = Rc::new(RefCell::new(Some("I have 42 apples and 7 oranges".to_string())));
    let mut matches = (*re.borrow_mut().as_mut().unwrap()).find_all_string(Rc::new(RefCell::new(Some((*text.borrow_mut().as_mut().unwrap())))), Rc::new(RefCell::new(Some(-1))));
    println!("{} {}", "Numbers found:".to_string(), format_slice(&matches));
}