use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Tracker {
    pub needs_channel: Rc<RefCell<Option<bool>>>,
    pub needs_context: Rc<RefCell<Option<bool>>>,
}

impl Tracker {
    pub fn __go_value_clone(&self) -> Self {
        Self { needs_channel: { let __guard = self.needs_channel.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, needs_context: { let __guard = self.needs_context.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Tracker {
    fn default() -> Self {
        Self { needs_channel: Rc::new(RefCell::new(Some(false))), needs_context: Rc::new(RefCell::new(Some(false))) }
    }
}

impl std::fmt::Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.needs_channel.borrow().as_ref().unwrap()), (*self.needs_context.borrow().as_ref().unwrap()))
    }
}


impl Tracker {
    pub fn without_shared(&self) -> Rc<RefCell<Option<Tracker>>> {
        if false {
        return Rc::new(RefCell::new(None));
    }
        let mut copy = Rc::new(RefCell::new(Some((*self).__go_value_clone())));
        { let new_val = false; *(*copy.borrow().as_ref().unwrap()).needs_channel.borrow_mut() = Some(new_val); };
        { let new_val = false; *(*copy.borrow().as_ref().unwrap()).needs_context.borrow_mut() = Some(new_val); };
        return copy.clone();
    }
}

fn main() {
    let mut tracker = Rc::new(RefCell::new(Some(Tracker { needs_channel: Rc::new(RefCell::new(Some(true))), needs_context: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    let mut copy = (*tracker.borrow_mut().as_mut().unwrap()).without_shared();
    println!("{} {} {} {}", (*(*tracker.borrow().as_ref().unwrap()).needs_channel.borrow().as_ref().unwrap()), (*(*tracker.borrow().as_ref().unwrap()).needs_context.borrow().as_ref().unwrap()), (*(*copy.borrow().as_ref().unwrap()).needs_channel.borrow().as_ref().unwrap()), (*(*copy.borrow().as_ref().unwrap()).needs_context.borrow().as_ref().unwrap()));
}