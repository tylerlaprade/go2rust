use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct logger {
}

impl logger {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for logger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl logger {
    pub fn trace(&self, format: Rc<RefCell<Option<String>>>, args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) {
        println!("{} {}", { let __v = (*format.borrow().as_ref().unwrap()).clone(); __v }, (*args.borrow().as_ref().unwrap()).len());
    }
}

fn main() {
    let mut l: Rc<RefCell<Option<logger>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*l.borrow().as_ref().unwrap()).trace(Rc::new(RefCell::new(Some("objects".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new("two".to_string()) as Box<dyn Any>]))));
}