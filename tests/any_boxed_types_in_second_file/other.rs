use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec, __go_type_name, go_any_clone};

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// The helpers that clone and name boxed values are generated into their own
/// file. They have to import what these payload types are spelled with, even
/// though main.go uses none of it.
pub fn type_name(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", __go_type_name(&**v.borrow().as_ref().unwrap())))))
}

pub fn report() {
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))]))));
    let mut failed = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        Rc::new(RefCell::new(None))
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>>)));
    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(counts.clone()) as Box<dyn Any>, Box::new(failed.clone()) as Box<dyn Any>, Box::new(3i64) as Box<dyn Any>])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        print!("{}\n", __go_type_name(item.as_ref()));
    } }
    println!("{}", format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}