use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut fields = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>>::new())));
    let mut typeName = Rc::new(RefCell::new(Some("Thing".to_string())));
    let mut fieldName = Rc::new(RefCell::new(Some("Name".to_string())));

    if (*(*fields.borrow().as_ref().unwrap()).get(&(*typeName.borrow().as_ref().unwrap()).clone()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow()).is_none() {
        { let __map_key = (*typeName.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::new()))); (*fields.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    { let __map_key = (*fieldName.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some("string".to_string()))); (*(*fields.borrow().as_ref().unwrap()).get(&(*typeName.borrow().as_ref().unwrap()).clone()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut conversions = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>::new())));
    { let __map_key = "Target".to_string(); let __map_value = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::new()))); (*conversions.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "Source".to_string(); let __map_value = Rc::new(RefCell::new(Some(true))); (*(*conversions.borrow().as_ref().unwrap()).get(&"Target".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut count = Rc::new(RefCell::new(Some(0)));
    for (targetName, sourceNames) in { let __range_holder = conversions.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if ((*sourceNames.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) == (0 as i32) {
        continue
    }
        if (*sourceNames.borrow().as_ref().unwrap()).get(&"Source".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false) {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + targetName.len() as i32); };
    }
    }

    let mut buckets = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<Vec<String>>>>>::new())));
    { let __map_key = 1; let __map_value = Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()]))); (*buckets.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __range_holder = (*buckets.borrow().as_ref().unwrap()).get(&1).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for name in __range_values.iter() {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + name.len() as i32); };
    } }

    let mut missing = (*fields.borrow().as_ref().unwrap()).get(&"Missing".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default());
    println!("{}", format!("{}", (*(*fields.borrow().as_ref().unwrap()).get(&"Thing".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).get(&"Name".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new())));
    println!("{}", format!("{}", (*missing.borrow()).is_none()));
    println!("{}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
}