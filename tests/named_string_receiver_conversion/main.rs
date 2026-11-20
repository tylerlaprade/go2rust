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
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialOrd for internalError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for internalError {}

impl Ord for internalError {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}

impl PartialEq<String> for internalError {
    fn eq(&self, other: &String) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialEq<&str> for internalError {
    fn eq(&self, other: &&str) -> bool {
        self.0.borrow().as_ref().unwrap().as_str() == *other
    }
}

impl std::ops::Add for internalError {
    type Output = internalError;
    fn add(self, other: Self) -> internalError {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other.0.borrow().as_ref().unwrap());
        internalError(Rc::new(RefCell::new(Some(__joined))))
    }
}

impl std::ops::Add<String> for internalError {
    type Output = internalError;
    fn add(self, other: String) -> internalError {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other);
        internalError(Rc::new(RefCell::new(Some(__joined))))
    }
}

impl std::ops::Add<&str> for internalError {
    type Output = internalError;
    fn add(self, other: &str) -> internalError {
        let __joined = format!("{}{}", self.0.borrow().as_ref().unwrap(), other);
        internalError(Rc::new(RefCell::new(Some(__joined))))
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