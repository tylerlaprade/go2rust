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
pub struct bag {
    pub values: Rc<RefCell<Option<Vec<i32>>>>,
}

impl bag {
    pub fn __go_value_clone(&self) -> Self {
        Self { values: self.values.clone() }
    }
}

impl std::fmt::Display for bag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.values))
    }
}


impl bag {
    pub fn sum(&self) -> Rc<RefCell<Option<i32>>> {
        let mut sum = Rc::new(RefCell::new(Some(0)));
        { let __range_holder = self.values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for value in __range_values.iter().copied() {
        { let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + value); };
    } }
        return sum.clone();
    }
}

fn main() {
    let mut bag = Rc::new(RefCell::new(Some(bag { values: Rc::new(RefCell::new(Some(vec![1, 2, 3]))), ..Default::default() })));
    println!("{}", format!("{}", (*(*bag.borrow().as_ref().unwrap()).sum().borrow().as_ref().unwrap())));
}