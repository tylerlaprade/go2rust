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
    let mut lines = Rc::new(RefCell::new(Some(vec!["a".to_string(), "".to_string(), "b".to_string(), "".to_string(), "c".to_string()])));
    let mut n = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = lines.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for line in __range_values.iter() {
        if (*line).clone() != "" {
        (*lines.borrow_mut().as_mut().unwrap())[((*n.borrow().as_ref().unwrap())) as usize] = line.clone();
        { let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
    { let new_val = Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = lines.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize..((*n.borrow().as_ref().unwrap())) as usize].to_vec() }))); lines = new_val; };
    println!("{}", format!("{}", format_slice(&lines)));
}