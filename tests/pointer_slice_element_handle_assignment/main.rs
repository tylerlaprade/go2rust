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
pub struct node {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct cache {
    pub items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<node>>>>>>>,
}

impl std::fmt::Display for cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped(&self.items))
    }
}


impl cache {
    pub fn store(&mut self, n: Rc<RefCell<Option<node>>>) {
        (*self.items.borrow_mut().as_mut().unwrap())[(0) as usize] = n.clone();
        println!("{}", (*(*(*self.items.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(cache { items: Rc::new(RefCell::new(Some(vec![Default::default(); (1) as usize]))), ..Default::default() })));
    let mut n = Rc::new(RefCell::new(Some(node { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    (*c.borrow_mut().as_mut().unwrap()).store(n.clone());
}