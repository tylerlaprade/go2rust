use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone, Default)]
pub struct holder {
    pub values: Rc<RefCell<Option<Vec<String>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { values: self.values.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.values))
    }
}


impl holder {
    pub fn has_values(&self) -> bool {
        { let __range_holder = self.values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for _ in __range_values.iter() {
        return true;
    } }
        false
    }

    pub fn count_with_check(&self) -> i32 {
        let mut count = Rc::new(RefCell::new(Some(0)));
        { let __range_holder = self.values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for _ in __range_values.iter() {
        if self.has_values() {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
        return (*count.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut h = Rc::new(RefCell::new(Some(holder { values: Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))), ..Default::default() })));
    println!("{}", format!("{}", (*h.borrow().as_ref().unwrap()).count_with_check()));
}