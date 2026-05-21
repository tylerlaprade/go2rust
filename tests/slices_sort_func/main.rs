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

pub fn compare_length(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<i32>>> {

    if ((*a.borrow().as_ref().unwrap()).len() as i32) < ((*b.borrow().as_ref().unwrap()).len() as i32) {
        return Rc::new(RefCell::new(Some(-1)));
    }
    if ((*a.borrow().as_ref().unwrap()).len() as i32) > ((*b.borrow().as_ref().unwrap()).len() as i32) {
        return Rc::new(RefCell::new(Some(1 as i32)));
    }
    if (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(-1)));
    }
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(1 as i32)));
    }
    return Rc::new(RefCell::new(Some(0 as i32)));
}

fn main() {
    let mut words = Rc::new(RefCell::new(Some(vec!["pear".to_string(), "fig".to_string(), "apple".to_string(), "plum".to_string(), "date".to_string()])));
    { let mut __sort_guard = words.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = compare_length(Rc::new(RefCell::new(Some(__a.clone()))), Rc::new(RefCell::new(Some(__b.clone())))); let __ord = (*__cmp.borrow().as_ref().unwrap()).cmp(&0); __ord }); } };
    println!("{}", format!("{}", format_slice(&words)));

    let mut numbers = Rc::new(RefCell::new(Some(vec![3, 1, 4, 2])));
    { let __cmp_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*b.borrow().as_ref().unwrap());
            let __tmp_y = (*a.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))); let mut __sort_guard = numbers.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = { let mut __cmp_guard = __cmp_holder.borrow_mut(); let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(Rc::new(RefCell::new(Some(__a.clone()))), Rc::new(RefCell::new(Some(__b.clone())))) }; let __ord = (*__cmp.borrow().as_ref().unwrap()).cmp(&0); __ord }); } };
    println!("{}", format!("{}", format_slice(&numbers)));
}