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
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_1_0,
        }
    }
}


impl Default for r#box {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            a: __go_default_0_0,
            b: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl r#box {
    pub fn rotate(&mut self, mut v: Rc<RefCell<Option<i32>>>) -> i32 {
        let mut old = Rc::new(RefCell::new(Some({ let __selector_holder = self.a.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        {
            let __tmp_0 = { let __selector_holder = self.b.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_1 = (*v.borrow().as_ref().unwrap()).clone();
            let __tmp_2 = { let __selector_holder = self.a.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            *self.a.borrow_mut() = Some(__tmp_0);
            *self.b.borrow_mut() = Some(__tmp_1);
            *v.borrow_mut() = Some(__tmp_2);
        };
        return (*old.borrow().as_ref().unwrap()) + (*self.a.borrow().as_ref().unwrap()) + (*self.b.borrow().as_ref().unwrap()) + (*v.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut x = Rc::new(RefCell::new(Some(r#box { a: Rc::new(RefCell::new(Some(1))), b: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut r = (*x.borrow_mut().as_mut().unwrap()).rotate(Rc::new(RefCell::new(Some(9))));
    {
            let __go_print_arg_0 = format!("{}", r);
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*(*x.borrow().as_ref().unwrap()).a.borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*(*x.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
}