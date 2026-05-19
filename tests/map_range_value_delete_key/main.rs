use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut ids = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([("real".to_string(), Rc::new(RefCell::new(Some(true)))), ("other".to_string(), Rc::new(RefCell::new(Some(true))))]))));
    let mut aliases = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("alias".to_string(), Rc::new(RefCell::new(Some("real".to_string()))))]))));

    for (_, id) in { let __range_holder = aliases.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let __map_handle = ids.clone(); let mut __map_guard = __map_handle.borrow_mut(); __map_guard.as_mut().unwrap().remove(&(*id.borrow().as_ref().unwrap()).clone()); };
    }

    println!("{} {}", format!("{}", (*ids.borrow().as_ref().unwrap()).get(&"real".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false)), format!("{}", (*ids.borrow().as_ref().unwrap()).get(&"other".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false)));
}