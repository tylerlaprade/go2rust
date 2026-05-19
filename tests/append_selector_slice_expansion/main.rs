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
pub struct Holder {
    pub items: Rc<RefCell<Option<Vec<String>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { items: self.items.clone() }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.items))
    }
}


fn main() {
    let mut holder = Rc::new(RefCell::new(Some(Holder { items: Rc::new(RefCell::new(Some(vec!["beta".to_string(), "gamma".to_string()]))), ..Default::default() })));
    let mut values = Rc::new(RefCell::new(Some(vec!["alpha".to_string()])));
    { let new_val = { let __append_target = values.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend({ let __slice_holder = (*holder.borrow().as_ref().unwrap()).items.clone(); let __slice_guard = __slice_holder.borrow(); (*__slice_guard.as_ref().unwrap()).clone() }.iter().cloned()); __append_target.clone() }; values = new_val; };
    println!("{} {}", format!("{}", (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*values.borrow().as_ref().unwrap())[(2) as usize].clone()));
}