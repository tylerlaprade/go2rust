use std::cell::{RefCell};
use std::rc::{Rc};

pub fn apply(r#fn: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>) -> Rc<RefCell<Option<i32>>> {

    return { let __f_guard = r#fn.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(4)))) };
}

fn main() {
    println!("{}", (*apply(Rc::new(RefCell::new(Some(Box::new(move |r#yield: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*r#yield.borrow().as_ref().unwrap());
            let __tmp_y = 1;
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))).borrow().as_ref().unwrap()));
}