use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i64, Rc<RefCell<Option<i64>>>>::from([(0i64, Rc::new(RefCell::new(Some(1i64))))]))));
    { let __map_holder = m.clone(); let __key = (*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64; let __rhs = 5; let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); };
    println!("{} {} {}", format!("{}", { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&0)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}