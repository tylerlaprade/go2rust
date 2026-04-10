use std::cell::{RefCell};
use std::rc::{Rc};

pub fn divmod(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x % __tmp_y)))
        });
}

fn main() {
    let mut a: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(1)));let mut b: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(2)));let mut s = Rc::new(RefCell::new(Some("go".to_string())));let mut t = Rc::new(RefCell::new(Some("rust".to_string())));let (mut q, mut r) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    println!("{} {} {} {} {} {}", (*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()), (*s.borrow().as_ref().unwrap()), (*t.borrow().as_ref().unwrap()), (*q.borrow().as_ref().unwrap()), (*r.borrow().as_ref().unwrap()));
}