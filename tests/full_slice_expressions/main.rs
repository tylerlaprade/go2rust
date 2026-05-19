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
    let mut nums = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    let mut all = Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = nums.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..].to_vec() })));
    println!("{} {} {}", format!("{}", (*all.borrow().as_ref().unwrap()).len()), format!("{}", (*all.borrow().as_ref().unwrap()).capacity()), format!("{}", format_slice(&all)));

    let mut s = Rc::new(RefCell::new(Some("hello".to_string())));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*s.borrow().as_ref().unwrap()).clone(); __s[..].to_string() }))).borrow().as_ref().unwrap())));
}