use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub fn pop(items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<item>>>>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let mut x = (*items.borrow().as_ref().unwrap())[(0) as usize].clone().clone();
    return Rc::new(RefCell::new(Some(Box::new(x.clone()) as Box<dyn Any>)));
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })))])));
    let mut p = ({
        let val = pop(items.clone()).clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Rc<RefCell<Option<item>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
    { let new_val = "beta".to_string(); *(*p.borrow().as_ref().unwrap()).name.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*(*items.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}