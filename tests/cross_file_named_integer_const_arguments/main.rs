mod types;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn take_reloc(k: Rc<RefCell<Option<RelocKind>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32)));
}

fn main() {
    let mut e: Rc<RefCell<Option<Encoder>>> = Rc::new(RefCell::new(Some(Default::default())));
    println!("{}", format!("{}", (*take_reloc(Rc::new(RefCell::new(Some(RelocKind(Rc::new(RefCell::new(Some(RELOC_META as i32)))))))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*(*e.borrow().as_ref().unwrap()).sync(Rc::new(RefCell::new(Some(SyncMarker(Rc::new(RefCell::new(Some(SYNC_BOOL as i32)))))))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*(*e.borrow().as_ref().unwrap()).call_sync().borrow().as_ref().unwrap())));
}