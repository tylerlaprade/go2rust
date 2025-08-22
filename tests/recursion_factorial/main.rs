use std::cell::{RefCell};
use std::rc::{Rc};

pub fn fact(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    if (*n.borrow_mut().as_mut().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some(1)));
    }
    return {
            let __tmp_x = (*n.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*fact(Rc::new(RefCell::new(Some((*n.borrow_mut().as_mut().unwrap()) - 1)))).borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

fn main() {
    println!("{}", (*fact(Rc::new(RefCell::new(Some(7)))).borrow().as_ref().unwrap()));
}