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
    let mut dst = Rc::new(RefCell::new(Some(vec![1, 2])));
    let mut src = Rc::new(RefCell::new(Some(vec![3, 4, 5])));
    {(*dst.borrow_mut()).get_or_insert_with(Vec::new).extend((*src.borrow().as_ref().unwrap()).iter().cloned()); dst.clone()};
    println!("{}", format_slice(&dst));

    let mut words = Rc::new(RefCell::new(Some(vec!["go".to_string(), "to".to_string(), "rust".to_string()])));
    let mut prefix = Rc::new(RefCell::new(Some(vec!["transpile".to_string()])));
    {(*prefix.borrow_mut()).get_or_insert_with(Vec::new).extend((*words.borrow().as_ref().unwrap()).iter().cloned()); prefix.clone()};
    println!("{}", format_slice(&prefix));
}