use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])));
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([]))));
    { let __map_key = "values".to_string(); let __map_value = Rc::new(RefCell::new(Some((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64))); (*counts.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map_holder = counts.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"values".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
}