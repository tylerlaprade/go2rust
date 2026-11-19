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
        let __go_clone_0_0 = { let __guard = self.arg.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.prob.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            arg: __go_clone_0_0,
            prob: __go_clone_1_0,
        }
    }
}


impl Default for argError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            arg: __go_default_0_0,
            prob: __go_default_1_0,
        }
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
        return (-(1), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("can't work with 42".to_string())))));
    }
    ((*arg.borrow().as_ref().unwrap()) + 3, Rc::new(RefCell::new(None)))
}

pub fn f2(arg: Rc<RefCell<Option<i32>>>) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    if (*arg.borrow().as_ref().unwrap()) == 42 {
        return (-(1), Rc::new(RefCell::new(Some(Box::new(argError { arg: Rc::new(RefCell::new(Some((*arg.borrow().as_ref().unwrap())))), prob: Rc::new(RefCell::new(Some("can't work with it".to_string()))), ..Default::default() }) as Box<dyn StdError>))));
    }
    ((*arg.borrow().as_ref().unwrap()) + 3, Rc::new(RefCell::new(None)))
}

fn main() {
    for i in vec![7, 42].iter().copied() {
        {
        let (mut r, mut e) = f1(Rc::new(RefCell::new(Some(i.clone()))));;
        if { let __nil_result = (*e.borrow()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "f1 failed:".to_string()), format!("{}", { let __text = match e.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        } else {
            println!("{} {}", format!("{}", "f1 worked:".to_string()), format!("{}", r));;
        }
    }
    }
    for i in vec![7, 42].iter().copied() {
        {
        let (mut r, mut e) = f2(Rc::new(RefCell::new(Some(i.clone()))));;
        if { let __nil_result = (*e.borrow()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "f2 failed:".to_string()), format!("{}", { let __text = match e.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
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
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(None::<argError>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<argError>)), false)
        }
    });;
        if ok {
            println!("{}", format!("{}", (*(*ae.borrow().as_ref().unwrap()).arg.borrow().as_ref().unwrap())));;
            println!("{}", format!("{}", (*(*ae.borrow().as_ref().unwrap()).prob.borrow().as_ref().unwrap()).clone()));;
        }
    }
}