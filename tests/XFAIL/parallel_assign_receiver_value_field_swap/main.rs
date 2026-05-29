use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct r#box {
    pub a: Rc<RefCell<Option<i32>>>,
    pub b: Rc<RefCell<Option<i32>>>,
}

impl r#box {
    pub fn __go_value_clone(&self) -> Self {
        Self { a: { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, b: { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for r#box {
    fn default() -> Self {
        Self { a: Rc::new(RefCell::new(Some(0))), b: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.a.borrow().as_ref().unwrap()), (*self.b.borrow().as_ref().unwrap()))
    }
}


impl r#box {
    pub fn rotate(&mut self, mut v: Rc<RefCell<Option<i32>>>) -> i32 {
        let mut old = Rc::new(RefCell::new(Some({ let __selector_holder = self.a.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let __tmp_0 = self.b.clone(); let __tmp_1 = (*v.borrow().as_ref().unwrap()); let __tmp_2 = self.a.clone(); *self.a.borrow_mut() = __tmp_0.borrow_mut().take(); *self.b.borrow_mut() = Some(__tmp_1); *v.borrow_mut() = __tmp_2.borrow_mut().take(); };
        return (*old.borrow().as_ref().unwrap()) + (*self.a.borrow().as_ref().unwrap()) + (*self.b.borrow().as_ref().unwrap()) + (*v.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut x = Rc::new(RefCell::new(Some(r#box { a: Rc::new(RefCell::new(Some(1 as i32))), b: Rc::new(RefCell::new(Some(2 as i32))), ..Default::default() })));
    let mut r = (*x.borrow_mut().as_mut().unwrap()).rotate(Rc::new(RefCell::new(Some(9))));
    eprintln!("{}", format!("{}", r));
    eprintln!("{}", format!("{}", (*(*x.borrow().as_ref().unwrap()).a.borrow().as_ref().unwrap())));
    eprintln!("{}", format!("{}", (*(*x.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap())));
}