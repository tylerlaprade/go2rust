use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Item {
    pub v: Rc<RefCell<Option<i32>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl Default for Item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            v: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.v.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn of(v: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Item>>> {
    Rc::new(RefCell::new(Some(Item { v: Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub fn make(r#static: Rc<RefCell<Option<[Item; 3]>>>, labels: Rc<RefCell<Option<Vec<Item>>>>) -> i32 {
    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = r#static.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        { let __rhs = (*item.v.borrow().as_ref().unwrap()); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    { let __range_holder = labels.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        { let __rhs = (*item.v.borrow().as_ref().unwrap()); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

impl GoValueClone for Item {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
