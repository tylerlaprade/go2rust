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

fn main() {
    let (mut a, mut b) = (Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))));
    classify(Rc::new(RefCell::new(None)), b.clone());
    classify(a.clone(), a.clone());
    classify(a.clone(), b.clone());
}