use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())));
    { let mut __map_guard = counts.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry("go".to_string()).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some(__value.as_ref().unwrap() + 1); }
    { let mut __map_guard = counts.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry("rust".to_string()).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some(__value.as_ref().unwrap() + 2); };
    println!("{} {} {}", format!("{}", (*counts.borrow().as_ref().unwrap()).get(&"go".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", (*counts.borrow().as_ref().unwrap()).get(&"rust".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", (*counts.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<bool>>>>::new())));
    { let __map_key = 10; let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = 20; let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {} {}", format!("{}", (*seen.borrow().as_ref().unwrap()).get(&10).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false)), format!("{}", (*seen.borrow().as_ref().unwrap()).get(&30).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false)), format!("{}", (*seen.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}