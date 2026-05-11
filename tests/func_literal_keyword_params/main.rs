use std::cell::{RefCell};
use std::rc::{Rc};

pub fn apply(r#fn: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>) -> Rc<RefCell<Option<i32>>> {

    return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = r#fn.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) };
}

fn main() {
    println!("{}", (*apply(Rc::new(RefCell::new(Some(Box::new(move |r#yield: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*r#yield.borrow().as_ref().unwrap());
            let __tmp_y = 1;
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))).borrow().as_ref().unwrap()));
}