use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut roots = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("/src".to_string(), Rc::new(RefCell::new(Some("module/path".to_string()))))]))));

    for (_, rpath) in { let __range_holder = roots.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if (*rpath.borrow().as_ref().unwrap()).clone() != "" {
        println!("{}", format!("{}", (*rpath.borrow_mut().as_mut().unwrap())));
    }
    }
}