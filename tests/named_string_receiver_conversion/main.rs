use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct internalError(pub Rc<RefCell<Option<String>>>);

impl Display for internalError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}

impl PartialEq for internalError {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}


impl internalError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", "gcimporter: ".to_string(), (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()).to_string()))).borrow().as_ref().unwrap())))));
    }
}

impl StdError for internalError {}


fn main() {
    let mut err = Rc::new(RefCell::new(Some(internalError(Rc::new(RefCell::new(Some("bad import data".to_string())))))));
    println!("{}", format!("{}", (*(*err.borrow().as_ref().unwrap()).error().borrow().as_ref().unwrap())));
}