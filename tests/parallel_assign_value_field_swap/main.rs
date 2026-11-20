use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct pair {
    pub begin: Rc<RefCell<Option<i64>>>,
    pub end: Rc<RefCell<Option<i64>>>,
}

impl pair {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.begin.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.end.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            begin: __go_clone_0_0,
            end: __go_clone_1_0,
        }
    }
}


impl Default for pair {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            begin: __go_default_0_0,
            end: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.begin.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.end.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


fn main() {
    let mut p = Rc::new(RefCell::new(Some(pair { begin: Rc::new(RefCell::new(Some(1))), end: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut x = Rc::new(RefCell::new(Some(9i64)));
    {
        let __tmp_0 = { let __selector_holder = (*p.borrow().as_ref().unwrap()).end.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_1 = (*x.borrow().as_ref().unwrap()).clone();
        let __tmp_2 = { let __selector_holder = (*p.borrow().as_ref().unwrap()).begin.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        *x.borrow_mut() = Some(__tmp_0);
        *(*p.borrow().as_ref().unwrap()).begin.borrow_mut() = Some(__tmp_1);
        *(*p.borrow().as_ref().unwrap()).end.borrow_mut() = Some(__tmp_2);
    };
    {
            let __go_print_arg_0 = format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*(*p.borrow().as_ref().unwrap()).begin.borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*(*p.borrow().as_ref().unwrap()).end.borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
}