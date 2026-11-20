use std::cell::{RefCell};
use std::rc::{Rc};

pub fn mark(label: Rc<RefCell<Option<String>>>, p: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<i64>>> {
    println!("{}", format!("{}", { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v }));
    p.clone()
}

pub fn classify(p: Rc<RefCell<Option<i64>>>, q: Rc<RefCell<Option<i64>>>) {
    { let _switch_val = mark(Rc::new(RefCell::new(Some("tag".to_string()))), p.clone());
    if (*_switch_val.borrow()).is_none() {
            println!("{}", format!("{}", "nil".to_string()));
        } else if { let __case = mark(Rc::new(RefCell::new(Some("first".to_string()))), q.clone()); let __same = Rc::ptr_eq(&_switch_val, &__case); if __same { true } else { let __tag_nil = _switch_val.borrow().is_none(); let __both_nil = __tag_nil && __case.borrow().is_none(); __both_nil } } || { let __case = mark(Rc::new(RefCell::new(Some("second".to_string()))), p.clone()); let __same = Rc::ptr_eq(&_switch_val, &__case); if __same { true } else { let __tag_nil = _switch_val.borrow().is_none(); let __both_nil = __tag_nil && __case.borrow().is_none(); __both_nil } } {
            println!("{}", format!("{}", "match".to_string()));
        } else {
            println!("{}", format!("{}", "default".to_string()));
        }
    }
}

pub fn shadow(p: Rc<RefCell<Option<i64>>>, _switch_val: Rc<RefCell<Option<i64>>>, _switch_val_: Rc<RefCell<Option<i64>>>) {
    { let _switch_val__ = p.clone();
    if (*_switch_val__.borrow()).is_none() {
            println!("{} {}", format!("{}", "nil".to_string()), format!("{}", { let __v = (*_switch_val_.borrow().as_ref().unwrap()).clone(); __v }));
        } else if { let __case = _switch_val.clone(); let __same = Rc::ptr_eq(&_switch_val__, &__case); if __same { true } else { let __tag_nil = _switch_val__.borrow().is_none(); let __both_nil = __tag_nil && __case.borrow().is_none(); __both_nil } } {
            println!("{} {}", format!("{}", "same".to_string()), format!("{}", { let __v = (*_switch_val_.borrow().as_ref().unwrap()).clone(); __v }));
        } else {
            println!("{} {}", format!("{}", "distinct".to_string()), format!("{}", { let __v = (*_switch_val.borrow().as_ref().unwrap()).clone(); __v }));
        }
    }
}

fn main() {
    let (mut a, mut b) = (Rc::new(RefCell::new(Some(1i64))), Rc::new(RefCell::new(Some(2i64))));
    shadow(a.clone(), b.clone(), Rc::new(RefCell::new(Some(7))));
    shadow(a.clone(), a.clone(), Rc::new(RefCell::new(Some(8))));
    classify(Rc::new(RefCell::new(None)), b.clone());
    classify(a.clone(), a.clone());
    classify(a.clone(), b.clone());
}