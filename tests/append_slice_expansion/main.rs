use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn main() {
    let mut dst = Rc::new(RefCell::new(Some(vec![1, 2])));
    let mut src = Rc::new(RefCell::new(Some(vec![3, 4, 5])));
    { let __append_target = dst.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend((*src.borrow().as_ref().unwrap()).clone().iter().cloned()); __append_target.clone() };
    println!("{}", format_slice(&dst));

    let mut words = Rc::new(RefCell::new(Some(vec!["go".to_string(), "to".to_string(), "rust".to_string()])));
    let mut prefix = Rc::new(RefCell::new(Some(vec!["transpile".to_string()])));
    { let __append_target = prefix.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend((*words.borrow().as_ref().unwrap()).clone().iter().cloned()); __append_target.clone() };
    println!("{}", format_slice(&prefix));
}