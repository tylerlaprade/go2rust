use crate::{format_any, go_format_any_nested};

use crate::{bbb_inner::{Inner}};

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone)]
pub struct Outer {
    pub name: Rc<RefCell<Option<String>>>,
    pub inner: Rc<RefCell<Option<Inner>>>,
}

impl Outer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.inner.clone();
        Self {
            name: __go_clone_0_0,
            inner: __go_clone_1_0,
        }
    }
}


impl Default for Outer {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            inner: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.inner.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
