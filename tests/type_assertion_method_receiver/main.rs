use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Signature {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Signature {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Signature {
    pub fn recv(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

pub fn recv_name(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    { let __recv = ({
        let val = v.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            Rc::new(RefCell::new(Some(any_val.downcast_ref::<Signature>().expect("type assertion failed").clone())))
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.borrow().as_ref().unwrap()).recv(); __result }
}

fn main() {
    println!("{}", format!("{}", (*recv_name(Rc::new(RefCell::new(Some(Box::new(Signature { name: Rc::new(RefCell::new(Some("receiver".to_string()))), ..Default::default() }) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}