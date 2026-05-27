use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct argError {
    pub arg: Rc<RefCell<Option<i32>>>,
    pub prob: Rc<RefCell<Option<String>>>,
}

impl argError {
    pub fn __go_value_clone(&self) -> Self {
        Self { arg: { let __guard = self.arg.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, prob: { let __guard = self.prob.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for argError {
    fn default() -> Self {
        Self { arg: Rc::new(RefCell::new(Some(0))), prob: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for argError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl argError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{} - {}", (*self.arg.borrow().as_ref().unwrap()), (*self.prob.borrow().as_ref().unwrap())))))
    }
}

impl StdError for argError {}


pub fn f1(arg: Rc<RefCell<Option<i32>>>) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*arg.borrow().as_ref().unwrap()) == 42 {
        return (-1, Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("can't work with 42".to_string())))));
    }
    ((*arg.borrow().as_ref().unwrap()) + 3, Rc::new(RefCell::new(None)))
}

pub fn f2(arg: Rc<RefCell<Option<i32>>>) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*arg.borrow().as_ref().unwrap()) == 42 {
        return (-1, Rc::new(RefCell::new(Some(Box::new(argError { arg: Rc::new(RefCell::new(Some((*arg.borrow().as_ref().unwrap())))), prob: Rc::new(RefCell::new(Some("can't work with it".to_string()))), ..Default::default() }) as Box<dyn StdError>))));
    }
    ((*arg.borrow().as_ref().unwrap()) + 3, Rc::new(RefCell::new(None)))
}

fn main() {
    for i in vec![7, 42].iter().copied() {
        {
        let (mut r, mut e) = f1(Rc::new(RefCell::new(Some(i.clone()))));;
        if (*e.borrow()).is_some() {
            println!("{} {}", format!("{}", "f1 failed:".to_string()), format!("{}", format!("{}", (*e.borrow().as_ref().unwrap()))));;
        } else {
            println!("{} {}", format!("{}", "f1 worked:".to_string()), format!("{}", r));;
        }
    }
    }
    for i in vec![7, 42].iter().copied() {
        {
        let (mut r, mut e) = f2(Rc::new(RefCell::new(Some(i.clone()))));;
        if (*e.borrow()).is_some() {
            println!("{} {}", format!("{}", "f2 failed:".to_string()), format!("{}", format!("{}", (*e.borrow().as_ref().unwrap()))));;
        } else {
            println!("{} {}", format!("{}", "f2 worked:".to_string()), format!("{}", r));;
        }
    }
    }

    let (_, mut e) = f2(Rc::new(RefCell::new(Some(42))));
    {
        let (mut ae, mut ok) = ({
        let val = e.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<argError>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            println!("{}", format!("{}", (*(*ae.borrow().as_ref().unwrap()).arg.borrow().as_ref().unwrap())));;
            println!("{}", format!("{}", (*(*ae.borrow().as_ref().unwrap()).prob.borrow().as_ref().unwrap()).clone()));;
        }
    }
}