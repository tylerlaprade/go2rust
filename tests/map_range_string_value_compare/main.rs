use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut roots = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("/src".to_string(), Rc::new(RefCell::new(Some("module/path".to_string()))))]))));

    for (_, rpath) in (*roots.borrow().as_ref().unwrap()).clone() {
        if (*rpath.borrow().as_ref().unwrap()).clone() != "" {
        println!("{}", (*rpath.borrow_mut().as_mut().unwrap()));
    }
    }
}