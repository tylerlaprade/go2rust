use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Clone)]
pub struct GoPtrKey<T>(pub Rc<RefCell<Option<T>>>);

impl<T> GoPtrKey<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { GoPtrKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoPtrKey<T> {}
impl<T> PartialOrd for GoPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Tuple;

impl std::fmt::Display for types_Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Tuple>")
    }
}


impl types_Tuple {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(Default::default())))
    }
}
