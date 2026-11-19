use std::cell::{RefCell};
use std::rc::{Rc};

pub fn mark(label: Rc<RefCell<Option<String>>>, p: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {
    println!("{}", format!("{}", { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v }));
    p.clone()
}

pub fn classify(p: Rc<RefCell<Option<i32>>>, q: Rc<RefCell<Option<i32>>>) {
    { let _switch_val = mark(Rc::new(RefCell::new(Some("tag".to_string()))), p.clone());
    if (*_switch_val.borrow()).is_none() {
            println!("{}", format!("{}", "nil".to_string()));
        } else if { let __case = mark(Rc::new(RefCell::new(Some("first".to_string()))), q.clone()); let __switch_guard = _switch_val.borrow(); let __case_guard = __case.borrow(); let __both_nil = (*__switch_guard).is_none() && (*__case_guard).is_none(); let __eq = __both_nil || Rc::ptr_eq(&_switch_val, &__case); __eq } || { let __case = mark(Rc::new(RefCell::new(Some("second".to_string()))), p.clone()); let __switch_guard = _switch_val.borrow(); let __case_guard = __case.borrow(); let __both_nil = (*__switch_guard).is_none() && (*__case_guard).is_none(); let __eq = __both_nil || Rc::ptr_eq(&_switch_val, &__case); __eq } {
            println!("{}", format!("{}", "match".to_string()));
        } else {
            println!("{}", format!("{}", "default".to_string()));
        }
    }
}

pub fn shadow(p: Rc<RefCell<Option<i32>>>, _switch_val: Rc<RefCell<Option<i32>>>, _switch_val_: Rc<RefCell<Option<i32>>>) {
    { let _switch_val__ = p.clone();
    if (*_switch_val__.borrow()).is_none() {
            println!("{} {}", format!("{}", "nil".to_string()), format!("{}", { let __v = (*_switch_val_.borrow().as_ref().unwrap()).clone(); __v }));
        } else if { let __case = _switch_val.clone(); let __switch_guard = _switch_val__.borrow(); let __case_guard = __case.borrow(); let __both_nil = (*__switch_guard).is_none() && (*__case_guard).is_none(); let __eq = __both_nil || Rc::ptr_eq(&_switch_val__, &__case); __eq } {
            println!("{} {}", format!("{}", "same".to_string()), format!("{}", { let __v = (*_switch_val_.borrow().as_ref().unwrap()).clone(); __v }));
        } else {
            println!("{} {}", format!("{}", "distinct".to_string()), format!("{}", { let __v = (*_switch_val.borrow().as_ref().unwrap()).clone(); __v }));
        }
    }
}

fn main() {
    let (mut a, mut b) = (Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))));
    shadow(a.clone(), b.clone(), Rc::new(RefCell::new(Some(7))));
    shadow(a.clone(), a.clone(), Rc::new(RefCell::new(Some(8))));
    classify(Rc::new(RefCell::new(None)), b.clone());
    classify(a.clone(), a.clone());
    classify(a.clone(), b.clone());
}