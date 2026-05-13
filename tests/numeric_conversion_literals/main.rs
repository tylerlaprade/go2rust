use std::cell::{RefCell};
use std::rc::{Rc};

pub fn byte_bit(i: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<u8>>> {

    return Rc::new(RefCell::new(Some((*Rc::new(RefCell::new(Some(1 as u8))).borrow().as_ref().unwrap()) << ((*i.borrow().as_ref().unwrap()) % 8))));
}

pub fn uint64_mask(i: Rc<RefCell<Option<u32>>>) -> Rc<RefCell<Option<u64>>> {

    return {
            let __tmp_x = (*Rc::new(RefCell::new(Some(1 as u64))).borrow().as_ref().unwrap());
            let __tmp_y = (*i.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x << __tmp_y)))
        };
}

pub fn byte_from_expr(v: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<u8>>> {

    return Rc::new(RefCell::new(Some(((*v.borrow().as_ref().unwrap()) + ('0' as u8)) as u8)));
}

fn main() {
    println!("{} {}", "byte literal bit:".to_string(), (*byte_bit(Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));
    println!("{} {}", "uint64 literal mask:".to_string(), (*uint64_mask(Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));
    println!("{} {}", "byte expression:".to_string(), (*byte_from_expr(Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap()));
}