use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut build = Rc::new(RefCell::new(Some(Box::new(move || -> (Rc<RefCell<Option<String>>>) {
    let mut value: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

        { let new_val = "named result".to_string(); *value.borrow_mut() = Some(new_val); };
        return Rc::new(RefCell::new(Some(value.borrow().as_ref().unwrap().clone())));
    }) as Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>)>)));
    println!("{}", format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>)> = { let mut __f_guard = build.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>)> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.borrow().as_ref().unwrap())));
}