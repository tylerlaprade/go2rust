use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub type reporter = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>>>>;


pub fn call(report: reporter) {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()> = { let mut __f_guard = report.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("ready".to_string()))), Rc::new(RefCell::new(Some(vec![])))) };
}

fn main() {
    call(Rc::new(RefCell::new(Some(Box::new(move |format: Rc<RefCell<Option<String>>>, args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>| {
        println!("{} {}", format!("{}", { let __v = (*format.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*args.borrow().as_ref().unwrap()).len()));
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>))));
}