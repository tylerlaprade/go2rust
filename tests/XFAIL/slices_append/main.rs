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
    let mut s: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(Some(Default::default())));
    {(*s.borrow_mut().as_mut().unwrap()).push("a".to_string()); s.clone()};
    {(*s.borrow_mut().as_mut().unwrap()).extend(vec!["b".to_string(), "c".to_string()]); s.clone()};
    println!("{} {}", "slice:".to_string(), format_slice(&s));

    let mut c = Rc::new(RefCell::new(Some(vec!["".to_string(); (*s.borrow().as_ref().unwrap()).len()])));
    copy(c.clone(), s.clone());
    println!("{} {}", "copy:".to_string(), format_slice(&c));

    let mut l = Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap())[2 as usize..5 as usize].to_vec())));
    println!("{} {}", "slice[2:5]:".to_string(), format_slice(&l));
}