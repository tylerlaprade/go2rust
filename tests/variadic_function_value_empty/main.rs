use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub type reporter = Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>>>>;


pub fn call(report: reporter) {
    { let __f_guard = report.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some("ready".to_string()))), Rc::new(RefCell::new(Some(vec![])))) };
}

fn main() {
    call(Rc::new(RefCell::new(Some(Box::new(move |format: Rc<RefCell<Option<String>>>, args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>| {
        println!("{} {}", { let __v = (*format.borrow().as_ref().unwrap()).clone(); __v }, (*args.borrow().as_ref().unwrap()).len());
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>))));
}