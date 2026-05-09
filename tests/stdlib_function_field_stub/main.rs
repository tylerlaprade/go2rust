use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct types_Config {
    pub error: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<Box<dyn StdError>>>>) -> ()>>>>,
}

impl std::fmt::Display for types_Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Config>")
    }
}


impl types_Config {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


fn main() {
    let _ = types_Config { error: Rc::new(RefCell::new(Some(Box::new(move |err: Rc<RefCell<Option<Box<dyn StdError>>>>| {
    }) as Box<dyn Fn(Rc<RefCell<Option<Box<dyn StdError>>>>) -> ()>))), ..Default::default() };
    println!("{}", "ok".to_string());
}