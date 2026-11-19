use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([("ready".to_string(), Rc::new(RefCell::new(Some(true))))]))));

    let (mut ready, mut okReady) = { let __map_holder = seen.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"ready".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(false))), false) } };
    let (mut missing, mut okMissing) = { let __map_holder = seen.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"missing".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(false))), false) } };

    println!("{} {}", format!("{}", { let __v = (*ready.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", okReady));
    println!("{} {}", format!("{}", { let __v = (*missing.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", okMissing));
}