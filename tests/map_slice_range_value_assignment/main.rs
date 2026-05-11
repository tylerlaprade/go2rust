use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut overlay = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<u8>>>>>::new())));
    { let __map_key = "file.go".to_string(); let __map_value = Rc::new(RefCell::new(Some(("go".to_string()).as_bytes().to_vec()))); (*overlay.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut src: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(None));
    for (filename, contents) in { let __range_holder = overlay.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        if filename.clone() == "file.go" {
        { let new_val = contents.clone(); src = new_val; };
        break
    }
    }

    println!("{}", (*src.borrow().as_ref().unwrap()).len());
    println!("{}", (*src.borrow().as_ref().unwrap())[(0) as usize].clone());
}