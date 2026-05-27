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
pub struct bucket {
    pub values: Rc<RefCell<Option<Vec<i32>>>>,
}

impl bucket {
    pub fn __go_value_clone(&self) -> Self {
        Self { values: self.values.clone() }
    }
}

impl std::fmt::Display for bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.values))
    }
}


impl bucket {
    pub fn has_room(&self) -> bool {
        return ((*self.values.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32) >= (3 as i32);
    }
}

fn main() {
    let mut b = Rc::new(RefCell::new(Some(bucket { values: Rc::new(RefCell::new(Some({ let mut v = Vec::with_capacity((3) as usize); v.resize((1) as usize, 0); v }))), ..Default::default() })));
    println!("{}", format!("{}", (*(*b.borrow().as_ref().unwrap()).values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    println!("{}", format!("{}", (*(*b.borrow().as_ref().unwrap()).values.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)));
    println!("{}", format!("{}", (*b.borrow().as_ref().unwrap()).has_room()));
}