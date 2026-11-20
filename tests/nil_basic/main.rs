use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic nil comparisons
    let mut p: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));
    if { let __nil_result = (*p.borrow()).is_none(); __nil_result } {
        println!("{}", format!("{}", "p is nil".to_string()));
    }

        // Assign nil
    let mut q: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    if { let __nil_result = (*q.borrow()).is_none(); __nil_result } {
        println!("{}", format!("{}", "q is nil".to_string()));
    }

        // Non-nil pointer
    let mut x = Rc::new(RefCell::new(Some(42i64)));
    { let new_val = x.clone().clone(); p = new_val; };
    if { let __nil_result = (*p.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "p is not nil, value:".to_string()), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));
    }

        // Set back to nil
    *p.borrow_mut() = None;
    if { let __nil_result = (*p.borrow()).is_none(); __nil_result } {
        println!("{}", format!("{}", "p is nil again".to_string()));
    }
}