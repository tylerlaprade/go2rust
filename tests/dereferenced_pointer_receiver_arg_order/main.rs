use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Counter {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Counter {
    pub fn set(&mut self, value: Rc<RefCell<Option<i32>>>) {
        { let new_val = value.borrow().as_ref().unwrap().clone(); *self.value.borrow_mut() = Some(new_val); };
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    { let __go_method_receiver = c.clone(); let __go_method_args = (Rc::new(RefCell::new(Some((*(*c.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()) + 1 as i32))),); let __go_method_result = (*__go_method_receiver.borrow_mut().as_mut().unwrap()).set(__go_method_args.0); __go_method_result };
    println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
}