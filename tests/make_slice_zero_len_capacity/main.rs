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
    let mut s = Rc::new(RefCell::new(Some(Vec::with_capacity(4))));
    println!("{} {} {}", (*s.borrow().as_ref().unwrap()).len(), (*s.borrow().as_ref().unwrap()).capacity(), (*s.borrow()).is_none());
    {(*s.borrow_mut()).get_or_insert_with(Vec::new).extend(vec![7, 8]); s.clone()};
    println!("{} {} {}", (*s.borrow().as_ref().unwrap()).len(), (*s.borrow().as_ref().unwrap()).capacity(), format_slice(&s));
}