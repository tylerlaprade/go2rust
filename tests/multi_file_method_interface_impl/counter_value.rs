use crate::{counter_step::{Counter}};

impl crate::counter_step::Counter {
    pub fn value(&self) -> i64 {
        return (*self.n.borrow().as_ref().unwrap());
    }
}