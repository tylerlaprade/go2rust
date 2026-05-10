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
    let mut env = Rc::new(RefCell::new(Some({ let mut v = Vec::with_capacity((2) as usize); v.resize((1) as usize, "".to_string()); v })));
    (*env.borrow_mut().as_mut().unwrap())[(0) as usize] = "A=1".to_string();

    let mut combined = { let __append_target = Rc::new(RefCell::new(Some((*env.borrow().as_ref().unwrap()).clone()))).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push("PWD=/tmp".to_string()); __append_target.clone() };
    (*combined.borrow_mut().as_mut().unwrap())[(0) as usize] = "B=2".to_string();

    println!("{}", format_slice(&env));
    println!("{}", format_slice(&combined));
}