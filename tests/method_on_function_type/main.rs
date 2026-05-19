use std::cell::{RefCell};
use std::rc::{Rc};

pub type formatter = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>>>>;


pub trait formatterMethods {
    fn format(&self, path: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>;
}

impl formatterMethods for formatter {
    fn format(&self, path: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = self.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(path.clone()) };
    }
}

fn main() {
    let mut f = Rc::new(RefCell::new(Some(Box::new(move |path: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", "pkg:".to_string(), (*path.borrow().as_ref().unwrap())))));
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>)));
    println!("{}", format!("{}", (*f.format(Rc::new(RefCell::new(Some("fmt".to_string())))).borrow().as_ref().unwrap())));
}