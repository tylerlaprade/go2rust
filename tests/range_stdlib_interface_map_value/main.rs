use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn values() -> Rc<RefCell<Option<Vec<types_Type>>>> {

    let mut typ: Rc<RefCell<Option<types_Type>>> = Rc::new(RefCell::new(None));
    return Rc::new(RefCell::new(Some(Vec::<types_Type>::from([(*typ.borrow().as_ref().unwrap()).clone()]))));
}

fn main() {
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<u64, Rc<RefCell<Option<types_Type>>>>::new())));
    if false {
        { let __range_holder = values().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, typ) in __range_values.iter().enumerate() {
        { let __map_key = { let __v = Rc::new(RefCell::new(Some(i as u64))); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }; let __map_value = Rc::new(RefCell::new(Some((*typ).clone()))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    }
    println!("{}", "ok".to_string());
}