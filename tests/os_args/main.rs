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

thread_local! {
    static __GO_OS_ARGS: std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Some(std::env::args().collect::<Vec<String>>())));
}

fn go_os_args() -> std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> {
    __GO_OS_ARGS.with(|args| args.clone())
}

fn main() {
    println!("{} {}", format!("{}", "Program name present:".to_string()), format!("{}", (*go_os_args().borrow().as_ref().unwrap())[(0) as usize].clone() != "".to_string()));
    println!("{} {}", format!("{}", "Arguments:".to_string()), format!("{}", format_slice(&Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = go_os_args().clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() }))))));
    println!("{} {}", format!("{}", "Total args:".to_string()), format!("{}", (*go_os_args().borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}