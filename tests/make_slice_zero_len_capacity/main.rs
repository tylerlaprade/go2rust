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
    let mut s = Rc::new(RefCell::new(Some(Vec::with_capacity((4) as usize))));
    println!("{} {} {}", (*s.borrow().as_ref().unwrap()).len(), (*s.borrow().as_ref().unwrap()).capacity(), (*s.borrow()).is_none());
    { let __append_target = s.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(vec![7, 8]); __append_target.clone() };
    println!("{} {} {}", (*s.borrow().as_ref().unwrap()).len(), (*s.borrow().as_ref().unwrap()).capacity(), format_slice(&s));
}