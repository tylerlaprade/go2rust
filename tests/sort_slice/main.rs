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
    let mut numbers = Rc::new(RefCell::new(Some(vec![64, 34, 25, 12, 22, 11, 90])));
    println!("{} {}", "Before:".to_string(), format_slice(&numbers));
    (*numbers.borrow_mut().as_mut().unwrap()).sort();
    println!("{} {}", "After:".to_string(), format_slice(&numbers));
}