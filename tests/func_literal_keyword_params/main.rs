use std::cell::{RefCell};
use std::rc::{Rc};

pub fn apply(r#fn: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>) -> i32 {
    { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = r#fn.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) }
}

fn main() {
    println!("{}", format!("{}", apply(Rc::new(RefCell::new(Some(Box::new(move |r#yield: Rc<RefCell<Option<i32>>>| -> i32 {
        (*r#yield.borrow().as_ref().unwrap()) + 1
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>))))));
}