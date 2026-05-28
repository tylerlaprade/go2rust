use std::cell::{RefCell};
use std::rc::{Rc};

/// Reproduces increment through a dereferenced pointer (`*p++`), as in
/// sort's `*swaps++`. The pointer operand is itself the wrapper handle, so
/// the mutation must lock the pointer once — not dereference to the bare
/// scalar place and then re-lock it (which calls .lock()/.borrow() on a
/// scalar: E0599).
pub fn bump(p: Rc<RefCell<Option<i32>>>) {
    { let mut guard = p.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let __rhs = 2; let mut guard = p.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
}

fn main() {
    let mut x = Rc::new(RefCell::new(Some(10)));
    bump(x.clone());
    println!("{}", format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
}