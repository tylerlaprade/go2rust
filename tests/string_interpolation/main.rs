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

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn main() {
    let mut name = Rc::new(RefCell::new(Some("World".to_string())));
    let mut age = Rc::new(RefCell::new(Some(25)));
    print!("Hello {}! You are {} years old.\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*age.borrow().as_ref().unwrap()).clone(); __v });
    let mut result = Rc::new(RefCell::new(Some(format!("Formatted: {}", format_slice(&Rc::new(RefCell::new(Some(vec![1, 2, 3]))))))));
    println!("{}", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });
}