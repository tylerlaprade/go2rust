use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct internalError(pub Rc<RefCell<Option<String>>>);

impl Display for internalError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for internalError {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}


impl internalError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()).to_string())));
    }
}

impl StdError for internalError {}


pub fn assigned() -> Rc<RefCell<Option<Box<dyn StdError>>>> {

    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));
    let mut ierr = Rc::new(RefCell::new(Some(internalError(Rc::new(RefCell::new(Some("assigned".to_string())))))));
    { let new_val = Box::new((*ierr.borrow().as_ref().unwrap()).clone()) as Box<dyn StdError>; *err.borrow_mut() = Some(new_val); };
    return err.clone();
}

pub fn direct() -> Rc<RefCell<Option<Box<dyn StdError>>>> {

    return Rc::new(RefCell::new(Some(Box::new(internalError(Rc::new(RefCell::new(Some("direct".to_string()))))) as Box<dyn StdError>)));
}

fn main() {
    println!("{}", format!("{}", (*(assigned()).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*(direct()).borrow().as_ref().unwrap())));
}