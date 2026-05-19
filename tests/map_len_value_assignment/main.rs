use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])));
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([]))));
    { let __map_key = "values".to_string(); let __map_value = Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap()).len() as i32))); (*counts.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", (*counts.borrow().as_ref().unwrap()).get(&"values".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));
}