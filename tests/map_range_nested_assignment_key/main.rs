use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn copy_nested(dst: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>>>>>, src: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>>>>>) {
    for (outerKey, srcInner) in { let __range_holder = src.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        if (*(*dst.borrow().as_ref().unwrap()).get(&outerKey).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow()).is_none() {
        { let __map_key = outerKey.clone(); let __map_value = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::new()))); (*dst.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        for (innerKey, value) in { let __range_holder = srcInner.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let __map_key = innerKey.clone(); let __map_value = Rc::new(RefCell::new(Some((*value.borrow().as_ref().unwrap()).clone()))); (*(*dst.borrow().as_ref().unwrap()).get(&outerKey).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    }
}

fn main() {
    let mut dst = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>>::from([]))));
    copy_nested(dst.clone(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>>::from([("outer".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("inner".to_string(), Rc::new(RefCell::new(Some("value".to_string()))))])))))])))));
    println!("{}", (*(*dst.borrow().as_ref().unwrap()).get(&"outer".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).get(&"inner".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));
}