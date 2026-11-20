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
        let __go_clone_0_0 = { let __guard = self.needs_channel.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.needs_context.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            needs_channel: __go_clone_0_0,
            needs_context: __go_clone_1_0,
        }
    }
}


impl Default for Tracker {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            needs_channel: __go_default_0_0,
            needs_context: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.needs_channel.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.needs_context.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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

    pub fn clear_context(&mut self) {
        { let new_val = false; *self.needs_context.borrow_mut() = Some(new_val); };
    }
}

fn main() {
    let mut tracker = Rc::new(RefCell::new(Some(Tracker { needs_channel: Rc::new(RefCell::new(Some(true))), needs_context: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    let mut copy = __go_nil_recv_tracker_without_shared({ let __recv = tracker.clone(); __recv });
    println!("{} {} {} {}", format!("{}", (*(*tracker.borrow().as_ref().unwrap()).needs_channel.borrow().as_ref().unwrap())), format!("{}", (*(*tracker.borrow().as_ref().unwrap()).needs_context.borrow().as_ref().unwrap())), format!("{}", (*(*copy.borrow().as_ref().unwrap()).needs_channel.borrow().as_ref().unwrap())), format!("{}", (*(*copy.borrow().as_ref().unwrap()).needs_context.borrow().as_ref().unwrap())));
    (*tracker.borrow_mut().as_mut().unwrap()).clear_context();
    println!("{}", format!("{}", (*(*tracker.borrow().as_ref().unwrap()).needs_context.borrow().as_ref().unwrap())));
}

pub fn __go_nil_recv_tracker_without_shared(t: Rc<RefCell<Option<Tracker>>>) -> Rc<RefCell<Option<Tracker>>> {
    if { let __nil_result = (*t.borrow()).is_none(); __nil_result } {
        return Rc::new(RefCell::new(None));
    }
    let mut copy = Rc::new(RefCell::new(Some({ let __v = (*t.borrow().as_ref().unwrap()).__go_value_clone(); __v })));
    { let new_val = false; *(*copy.borrow().as_ref().unwrap()).needs_channel.borrow_mut() = Some(new_val); };
    { let new_val = false; *(*copy.borrow().as_ref().unwrap()).needs_context.borrow_mut() = Some(new_val); };
    return copy.clone();
}