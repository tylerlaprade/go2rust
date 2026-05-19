use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([("ready".to_string(), Rc::new(RefCell::new(Some(true))))]))));

    let (mut ready, mut okReady) = match (*seen.borrow().as_ref().unwrap()).get(&"ready".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(false))), Rc::new(RefCell::new(Some(false)))) };
    let (mut missing, mut okMissing) = match (*seen.borrow().as_ref().unwrap()).get(&"missing".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(false))), Rc::new(RefCell::new(Some(false)))) };

    println!("{} {}", format!("{}", { let __v = (*ready.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*okReady.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", { let __v = (*missing.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*okMissing.borrow().as_ref().unwrap()).clone(); __v }));
}