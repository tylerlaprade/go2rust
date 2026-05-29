use crate::counter_step::*;
use crate::iface::*;

impl crate::counter_step::Counter {
    pub fn value(&self) -> i32 {
        return (*self.n.borrow().as_ref().unwrap());
    }
}