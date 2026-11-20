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

impl PartialOrd for internalError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
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
        Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()).clone())))
    }
}

impl StdError for internalError {}


pub fn assigned() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));
    let mut ierr = Rc::new(RefCell::new(Some(internalError(Rc::new(RefCell::new(Some("assigned".to_string())))))));
    { let new_val = Rc::new((*ierr.borrow().as_ref().unwrap()).clone()) as Rc<dyn StdError>; *err.borrow_mut() = Some(new_val); };
    return { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
}

pub fn direct() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Rc::new(internalError(Rc::new(RefCell::new(Some("direct".to_string()))))) as Rc<dyn StdError>)))
}

fn main() {
    println!("{}", format!("{}", { let __text = match assigned().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", { let __text = match direct().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}