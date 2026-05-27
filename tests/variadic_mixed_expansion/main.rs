use std::cell::{RefCell};
use std::rc::{Rc};

pub fn join(prefix: Rc<RefCell<Option<String>>>, sep: Rc<RefCell<Option<String>>>, parts: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {
    let mut out = Rc::new(RefCell::new(Some(prefix.borrow().as_ref().unwrap().clone())));
    { let __range_holder = parts.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", (*sep.borrow().as_ref().unwrap()), p)); };
    } }
    return Rc::new(RefCell::new(Some(out.borrow().as_ref().unwrap().clone())));
}

fn main() {
    let mut rest = Rc::new(RefCell::new(Some(vec!["b".to_string(), "c".to_string()])));
    println!("{}", format!("{}", (*join(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(":".to_string()))), rest.clone()).borrow().as_ref().unwrap())));
}