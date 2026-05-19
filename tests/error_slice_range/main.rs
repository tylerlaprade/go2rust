use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct customError {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl customError {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for customError {
    fn default() -> Self {
        Self { msg: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for customError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl customError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return self.msg.clone();
    }
}

impl StdError for customError {}


pub fn accept(err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*err.borrow()).is_none() {
        println!("{}", "nil".to_string());
        return;
    }
    println!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap()));
}

pub fn collect() -> Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn StdError>>>>>>>> {

    let mut errs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string())))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))])));
    (*errs.borrow_mut().as_mut().unwrap())[(1) as usize] = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("two".to_string()))));
    { let new_val = { let __append_target = errs.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(Rc::new(RefCell::new(Some(Box::new(customError { msg: Rc::new(RefCell::new(Some("custom".to_string()))), ..Default::default() }) as Box<dyn StdError>)))); __append_target.clone() }; errs = new_val; };
    return errs.clone();
}

fn main() {
    { let __range_holder = collect().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for err in __range_values.iter().cloned() {
        accept(err.clone());
    } }
}