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
pub struct source {
    pub files: Rc<RefCell<Option<Vec<String>>>>,
}

impl source {
    pub fn __go_value_clone(&self) -> Self {
        Self { files: self.files.clone() }
    }
}

impl std::fmt::Display for source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.files))
    }
}


pub fn flatten(groups: Rc<RefCell<Option<Vec<Vec<String>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut out: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = groups.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for group in __range_values.iter() {
        { let new_val = { let __append_target = out.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(group.iter().cloned()); __append_target.clone() }; out = new_val; };
    } }
    return out.clone();
}

fn main() {
    let mut src: Rc<RefCell<Option<source>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut groups = Rc::new(RefCell::new(Some(vec![{ let __slice_holder = (*src.borrow().as_ref().unwrap()).files.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, vec!["go".to_string()]])));
    println!("{}", format!("{}", (*groups.borrow().as_ref().unwrap())[(0) as usize].clone().len()));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*flatten(groups.clone()).borrow()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
}