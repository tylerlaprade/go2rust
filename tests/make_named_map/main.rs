use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Counts(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>);


fn main() {
    let mut m = Rc::new(RefCell::new(Some(Counts(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())))))));
    { let __map_key = "a".to_string(); let __map_value = Rc::new(RefCell::new(Some(1))); (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "b".to_string(); let __map_value = Rc::new(RefCell::new(Some(2))); (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "c".to_string(); let __map_value = Rc::new(RefCell::new(Some(3))); (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut keys = Rc::new(RefCell::new(Some(Vec::with_capacity(({ let __map_holder = { let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }; let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize))));
    for (k, _) in { let __range_holder = { let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }; let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(k.clone()); __append_target.clone() }; keys = new_val; };
    }
    { let mut __sort_guard = keys.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for k in __range_values.iter() {
        println!("{} {}", format!("{}", k), format!("{}", (*{ let __named_map = (*m.borrow().as_ref().unwrap()).0.clone(); __named_map }.borrow().as_ref().unwrap()).get(k).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));
    } }
}