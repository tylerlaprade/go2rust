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

#[derive(Debug, Clone, Default)]
pub struct Item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Holder {
    pub item: Rc<RefCell<Option<Item>>>,
    pub values: Rc<RefCell<Option<Vec<i32>>>>,
}


impl Default for Holder {
    fn default() -> Self {
        Self { item: Rc::new(RefCell::new(Some(Item::default()))), values: Default::default() }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.item.borrow().as_ref().unwrap()), format_slice(&self.values))
    }
}


pub fn get_item(h: Rc<RefCell<Option<Holder>>>) -> Rc<RefCell<Option<Item>>> {

    return Rc::new(RefCell::new(Some((*(*h.borrow().as_ref().unwrap()).item.borrow().as_ref().unwrap()).clone())));
}

pub fn get_values(h: Rc<RefCell<Option<Holder>>>) -> Rc<RefCell<Option<Vec<i32>>>> {

    return Rc::new(RefCell::new(Some((*(*h.borrow().as_ref().unwrap()).values.borrow().as_ref().unwrap()).clone())));
}

fn main() {
    let mut h = Rc::new(RefCell::new(Some(Holder { item: Rc::new(RefCell::new(Some(Item { name: Rc::new(RefCell::new(Some("go".to_string()))), ..Default::default() }))), values: Rc::new(RefCell::new(Some(vec![2, 3]))), ..Default::default() })));

    let mut item = get_item(Rc::new(RefCell::new(Some((*h.borrow().as_ref().unwrap()).clone()))));
    let mut values = get_values(Rc::new(RefCell::new(Some((*h.borrow().as_ref().unwrap()).clone()))));

    println!("{}", (*(*item.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    println!("{} {} {}", (*values.borrow().as_ref().unwrap()).len(), (*values.borrow().as_ref().unwrap())[(0) as usize].clone(), (*values.borrow().as_ref().unwrap())[(1) as usize].clone());
}