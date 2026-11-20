use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Default)]
pub struct internalError(pub Rc<RefCell<Option<String>>>);

impl Clone for internalError {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        internalError(Rc::new(RefCell::new(__value)))
    }
}

impl Display for internalError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}

impl PartialEq for internalError {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}


impl internalError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "gcimporter: ".to_string(), (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()).clone()))).borrow().as_ref().unwrap())))))
    }
}

impl StdError for internalError {}


fn main() {
    let mut err = Rc::new(RefCell::new(Some(internalError(Rc::new(RefCell::new(Some("bad import data".to_string())))))));
    println!("{}", format!("{}", (*(*err.borrow().as_ref().unwrap()).error().borrow().as_ref().unwrap())));
}