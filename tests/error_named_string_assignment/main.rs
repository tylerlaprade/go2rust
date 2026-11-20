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