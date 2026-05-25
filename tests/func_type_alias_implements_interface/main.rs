use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

pub trait Speaker: std::fmt::Display + Any {
    fn __go_clone_box_speaker(&self) -> Box<dyn Speaker>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_speaker(&self, other: &dyn Speaker) -> bool;
    fn speak(&self) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn Speaker> {
    fn clone(&self) -> Self {
        self.__go_clone_box_speaker()
    }
}

pub type counter = Rc<RefCell<Option<Box<dyn FnMut() -> Rc<RefCell<Option<i32>>>>>>>;

#[derive(Clone)]
pub struct counterAsSpeaker(pub counter);

impl std::fmt::Display for counterAsSpeaker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<counterAsSpeaker>")
    }
}

impl Speaker for counterAsSpeaker {
    fn speak(&self) -> Rc<RefCell<Option<i32>>> {
        self.0.speak()
    }
    fn __go_clone_box_speaker(&self) -> Box<dyn Speaker> {
        Box::new(self.clone())
    }
    fn __go_as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn __go_eq_speaker(&self, _other: &dyn Speaker) -> bool {
        false
    }
}


pub trait counterMethods {
    fn speak(&self) -> Rc<RefCell<Option<i32>>>;
}

impl counterMethods for counter {
    fn speak(&self) -> Rc<RefCell<Option<i32>>> {
        return { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = self.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
}

pub fn run_speaker(s: Rc<RefCell<Option<Box<dyn Speaker>>>>) -> Rc<RefCell<Option<i32>>> {

    return (*s.borrow().as_ref().unwrap()).speak();
}

pub fn make_counter() -> counter {

    let mut x = Rc::new(RefCell::new(Some(41)));
    let x_closure_clone = x.clone(); return Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<i32>>> {
        { let mut guard = x_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return Rc::new(RefCell::new(Some(x_closure_clone.borrow().as_ref().unwrap().clone())));
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<i32>>>>)));
}

fn main() {
    let mut c = make_counter();
    println!("{}", format!("{}", (*run_speaker(Rc::new(RefCell::new(Some(Box::new(counterAsSpeaker(c.clone())) as Box<dyn Speaker>)))).borrow().as_ref().unwrap())));
}