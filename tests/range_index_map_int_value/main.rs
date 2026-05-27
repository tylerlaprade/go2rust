use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn roots() -> Rc<RefCell<Option<Vec<String>>>> {
    Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()])))
}

fn main() {
    let mut rootMap = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([]))));
    { let __range_holder = roots().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, root) in __range_values.iter().enumerate() {
        { let __map_key = (*root).clone(); let __map_value = Rc::new(RefCell::new(Some(i as i32))); (*rootMap.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    println!("{}", format!("{}", (*rootMap.borrow().as_ref().unwrap()).get(&"alpha".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));
    println!("{}", format!("{}", (*rootMap.borrow().as_ref().unwrap()).get(&"beta".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));
}