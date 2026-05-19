use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn values() -> Rc<RefCell<Option<Vec<String>>>> {

    return Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])));
}

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<u64>>>>::from([]))));
    { let __range_holder = values().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, v) in __range_values.iter().enumerate() {
        { let __map_key = (*v).clone(); let __map_value = Rc::new(RefCell::new(Some(i as u64))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    println!("{} {}", format!("{}", (*m.borrow().as_ref().unwrap()).get(&"a".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", (*m.borrow().as_ref().unwrap()).get(&"b".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));
}