use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn nil_map() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>> {
    Rc::new(RefCell::new(None))
}

pub fn count_entries(values: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) -> i32 {
    let mut count = Rc::new(RefCell::new(Some(0)));
    for (key, value) in { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        println!("{} {} {}", format!("{}", "unexpected".to_string()), format!("{}", key), format!("{}", (*value.borrow_mut().as_mut().unwrap())));
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (*count.borrow().as_ref().unwrap());
}

fn main() {
    println!("{}", format!("{}", count_entries(nil_map())));
}