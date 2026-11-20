use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut fields = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>>::new())));
    let mut typeName = Rc::new(RefCell::new(Some("Thing".to_string())));
    let mut fieldName = Rc::new(RefCell::new(Some("Name".to_string())));

    if { let __nil_result = (*{ let __map_holder = fields.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&(*typeName.borrow().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow()).is_none(); __nil_result } {
        { let __map_key = (*typeName.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::new()))); (*fields.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    { let __map_key = (*fieldName.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some("string".to_string()))); (*{ let __map_holder = fields.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&(*typeName.borrow().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut conversions = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>::new())));
    { let __map_key = "Target".to_string(); let __map_value = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::new()))); (*conversions.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "Source".to_string(); let __map_value = Rc::new(RefCell::new(Some(true))); (*{ let __map_holder = conversions.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"Target".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut count = Rc::new(RefCell::new(Some(0i64)));
    for (targetName, sourceNames) in { let __range_holder = conversions.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if ((*sourceNames.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) == (0 as i64) {
        continue
    }
        if { let __map_holder = sourceNames.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"Source".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        { let __rhs = targetName.len() as i64; let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }

    let mut buckets = Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<Vec<String>>>>>::new())));
    { let __map_key = 1; let __map_value = Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()]))); (*buckets.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __range_holder = { let __map_holder = buckets.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        { let __rhs = name.len() as i64; let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }

    let mut missing = { let __map_holder = fields.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"Missing".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
    println!("{}", format!("{}", { let __map_holder = { let __map_holder = fields.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"Thing".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"Name".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    println!("{}", format!("{}", { let __nil_result = (*missing.borrow()).is_none(); __nil_result }));
    println!("{}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
}