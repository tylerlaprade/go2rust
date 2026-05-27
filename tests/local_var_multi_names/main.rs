use std::cell::{RefCell};
use std::rc::{Rc};

pub fn divmod(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (i32, i32) {
    ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap()))
}

fn main() {
    let mut a: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(1)));let mut b: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(2)));let mut s = Rc::new(RefCell::new(Some("go".to_string())));let mut t = Rc::new(RefCell::new(Some("rust".to_string())));let (mut q, mut r) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    println!("{} {} {} {} {} {}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*t.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*r.borrow().as_ref().unwrap()).clone(); __v }));
}