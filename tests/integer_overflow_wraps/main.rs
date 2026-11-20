use std::cell::{RefCell};
use std::rc::{Rc};

pub mod math {
    use super::*;
    pub const MAX_INT32: i64 = 2147483647;
    pub const MAX_INT64: i64 = 9223372036854775807;
}


pub fn fnv32(s: Rc<RefCell<Option<String>>>) -> u32 {
    let mut h = Rc::new(RefCell::new(Some(2166136261 as u32)));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while ((*i.borrow().as_ref().unwrap()) as i64) < ((*s.borrow().as_ref().unwrap()).len() as i64) {
        { let __rhs = (*Rc::new(RefCell::new(Some({ let __s = &((*s.borrow().as_ref().unwrap()).clone()); __s.as_bytes()[((*i.borrow().as_ref().unwrap())) as usize] } as u32))).borrow().as_ref().unwrap()); let mut guard = h.borrow_mut(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let __rhs = 16777619 as u32; let mut guard = h.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (*h.borrow().as_ref().unwrap());
}

fn main() {
    let mut x: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(math::MAX_INT32 as i32)));
    { let mut guard = x.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    let mut y: Rc<RefCell<Option<i8>>> = Rc::new(RefCell::new(Some(-128 as i8)));
    { let mut guard = y.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    let mut u: Rc<RefCell<Option<u8>>> = Rc::new(RefCell::new(Some(250)));
    { let __rhs = 10 as u8; let mut guard = u.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut z: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(math::MAX_INT64 as i64)));
    { let __rhs = 2 as i64; let mut guard = z.borrow_mut(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    println!("{} {} {} {}", format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*u.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*z.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", fnv32(Rc::new(RefCell::new(Some("hello".to_string()))))), format!("{}", fnv32(Rc::new(RefCell::new(Some("go2rust".to_string()))))));
    let mut w: Rc<RefCell<Option<u16>>> = Rc::new(RefCell::new(Some(0)));
    { let __rhs = 1 as u16; let mut guard = w.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    println!("{}", format!("{}", { let __v = (*w.borrow().as_ref().unwrap()).clone(); __v }));
}