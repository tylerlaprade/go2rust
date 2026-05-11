use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut overlay = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<u8>>>>>::from([("file.go".to_string(), Rc::new(RefCell::new(None)))]))));
    let mut filename = Rc::new(RefCell::new(Some("file.go".to_string())));
    let mut goFiles: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    let mut compiledGoFiles: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (path, _) in { let __range_holder = overlay.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        if path.clone() == (*filename.borrow().as_ref().unwrap()) {
        { let new_val = Rc::new(RefCell::new(Some(vec![path.clone()]))); goFiles = new_val; };
        { let new_val = Rc::new(RefCell::new(Some(vec![path.clone()]))); compiledGoFiles = new_val; };
    }
    }
    println!("{} {}", (*goFiles.borrow().as_ref().unwrap())[(0) as usize].clone(), (*compiledGoFiles.borrow().as_ref().unwrap())[(0) as usize].clone());
}