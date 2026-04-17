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
    let mut nums = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    let mut all = Rc::new(RefCell::new(Some((*nums.borrow().as_ref().unwrap())[..].to_vec())));
    println!("{} {} {}", (*all.borrow().as_ref().unwrap()).len(), (*all.borrow().as_ref().unwrap()).capacity(), format_slice(&all));

    let mut s = Rc::new(RefCell::new(Some("hello".to_string())));
    println!("{}", (*Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[..].to_string()))).borrow().as_ref().unwrap()));
}