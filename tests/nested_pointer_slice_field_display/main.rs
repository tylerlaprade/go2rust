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


fn format_slice_wrapped_values<T>(slice: &[Rc<RefCell<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.borrow();
        match inner.as_ref() {
            Some(value) => format!("&{}", value),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}
fn format_nested_slice_wrapped<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s
            .as_ref()
            .iter()
            .map(|inner| format_slice_wrapped_values(inner.as_ref()))
            .collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct resolver {
    pub stack: Rc<RefCell<Option<Vec<Vec<Rc<RefCell<Option<Ident>>>>>>>>,
}

impl resolver {
    pub fn __go_value_clone(&self) -> Self {
        Self { stack: self.stack.clone() }
    }
}

impl std::fmt::Display for resolver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_nested_slice_wrapped(&self.stack))
    }
}


fn main() {
    let mut a = Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() })));
    let mut r = Rc::new(RefCell::new(Some(resolver { stack: Rc::new(RefCell::new(Some(vec![vec![a.clone(), b.clone()], vec![a.clone()]]))), ..Default::default() })));
    eprintln!("{}", format!("{}", (*(*r.borrow().as_ref().unwrap()).stack.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    eprintln!("{}", format!("{}", (*(*r.borrow().as_ref().unwrap()).stack.borrow().as_ref().unwrap())[(0) as usize].clone().len()));
    eprintln!("{}", format!("{}", (*(*(*(*r.borrow().as_ref().unwrap()).stack.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone().borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}