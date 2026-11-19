use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct info {
    pub name: Rc<RefCell<Option<String>>>,
}

impl info {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for info {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn accept(ptr: Rc<RefCell<Option<info>>>) -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some({ let __selector_holder = (*ptr.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

pub fn r#box(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    value.clone()
}

fn main() {
    let mut value = r#box(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(info { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    println!("{}", format!("{}", (*accept(({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Rc<RefCell<Option<info>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    })).borrow().as_ref().unwrap())));
}