mod types;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn decode(ver: Rc<RefCell<Option<u32>>>, flags: Rc<RefCell<Option<u32>>>) -> Rc<RefCell<Option<bool>>> {

    let mut h: Rc<RefCell<Option<Header>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = Version(Rc::new(RefCell::new(Some((*ver.borrow().as_ref().unwrap()) as u32)))); *(*h.borrow().as_ref().unwrap()).version.borrow_mut() = Some(new_val); };
    if (*(*h.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()) >= Version(Rc::new(RefCell::new(Some(NUM_VERSIONS as u32)))) {
        return Rc::new(RefCell::new(Some(false)));
    }
    return Rc::new(RefCell::new(Some((*(*(*h.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()).has(Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(V1 as u32)))))))).borrow().as_ref().unwrap()) && (*flags.borrow().as_ref().unwrap()) & FLAG_SYNC_MARKERS as u32 != 0 as u32)));
}

fn main() {
    println!("{}", (*decode(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap()));
    println!("{}", (*decode(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap()));
    println!("{}", (*decode(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap()));
}