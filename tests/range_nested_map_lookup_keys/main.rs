use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut index = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<u64>>>>>>>>::from([("dep".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<u64>>>>::from([("beta".to_string(), Rc::new(RefCell::new(Some(2)))), ("alpha".to_string(), Rc::new(RefCell::new(Some(1))))])))))]))));

    let mut names = Rc::new(RefCell::new(Some(Vec::with_capacity(((*(*index.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (name, _) in { let __range_holder = (*index.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = names.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(name.clone()); __append_target.clone() }; names = new_val; };
    }
    { let mut __sort_guard = names.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {}", format!("{}", (*names.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*names.borrow().as_ref().unwrap())[(1) as usize].clone()));
}