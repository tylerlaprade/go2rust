use std::cell::{RefCell};
use std::rc::{Rc};

pub fn equal(p: Rc<RefCell<Option<i64>>>, q: Rc<RefCell<Option<i64>>>) -> bool {
    { let _switch_val = p.clone();
    if { let __case = q.clone(); let __same = Rc::ptr_eq(&_switch_val, &__case); if __same { true } else { let __tag_nil = _switch_val.borrow().is_none(); let __both_nil = __tag_nil && __case.borrow().is_none(); __both_nil } } {
            return true;
        }
    }
    false
}

fn main() {
    let (mut a, mut b) = (Rc::new(RefCell::new(Some(3i64))), Rc::new(RefCell::new(Some(3i64))));
    let (mut p, mut q) = (a.clone(), b.clone());
    println!("{} {} {} {} {}", format!("{}", equal(p.clone(), q.clone())), format!("{}", equal(p.clone(), p.clone())), format!("{}", equal(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)))), format!("{}", equal(p.clone(), Rc::new(RefCell::new(None)))), format!("{}", equal(Rc::new(RefCell::new(None)), q.clone())));
}