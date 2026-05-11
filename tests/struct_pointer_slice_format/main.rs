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
pub struct item {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<item>>>>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { items: self.items.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped(&self.items))
    }
}


fn main() {
    let mut h = Rc::new(RefCell::new(Some(holder { items: Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(item { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(item { n: Rc::new(RefCell::new(Some(2))), ..Default::default() })))]))), ..Default::default() })));
    let _ = Rc::new(RefCell::new(Some(format!("{}", (*h.borrow().as_ref().unwrap())))));
    println!("{}", "ok".to_string());
}