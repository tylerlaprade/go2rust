use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct types_Error {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for types_Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Error>")
    }
}

impl std::error::Error for types_Error {}


impl types_Error {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(Default::default())))
    }
}


pub fn accept(err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*err.borrow()).is_some() {
        println!("{}", "ok".to_string());
    }
}

fn main() {
    let mut err = types_Error { msg: Rc::new(RefCell::new(Some("boom".to_string()))), ..Default::default() };
    accept(Rc::new(RefCell::new(Some(Box::new(err) as Box<dyn StdError>))));
}