use std::cell::{RefCell};
use std::rc::{Rc};

pub fn divmod(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap()))
}

fn main() {
    let mut a: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(1)));let mut b: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(2)));let mut s = Rc::new(RefCell::new(Some("go".to_string())));let mut t = Rc::new(RefCell::new(Some("rust".to_string())));let (mut q, mut r) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    println!("{} {} {} {} {} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*t.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", q), format!("{}", r));
}