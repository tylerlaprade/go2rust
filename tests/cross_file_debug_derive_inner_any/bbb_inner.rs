use crate::{format_any, go_format_any_nested};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone)]
pub struct Inner {
    pub tag: Rc<RefCell<Option<String>>>,
    pub data: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Inner {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.tag.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.data.clone();
        Self {
            tag: __go_clone_0_0,
            data: __go_clone_1_0,
        }
    }
}


impl Default for Inner {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            tag: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.tag.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.data.borrow().as_ref() { Some(__v) => go_format_any_nested(|| format_any(__v.as_ref())), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
