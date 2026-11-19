use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct ImportSpec {
    pub path: Rc<RefCell<Option<String>>>,
}

impl ImportSpec {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.path.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            path: __go_clone_0_0,
        }
    }
}


impl Default for ImportSpec {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            path: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for ImportSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.path.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut imports = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(ImportSpec { path: Rc::new(RefCell::new(Some("fmt".to_string()))), ..Default::default() }))), Rc::new(RefCell::new(Some(ImportSpec { path: Rc::new(RefCell::new(Some("os".to_string()))), ..Default::default() })))])));
    { let __range_holder = imports.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut s in __range_values.iter().cloned() {
        let mut s = s.clone();
        println!("{}", format!("{}", (*(*s.borrow().as_ref().unwrap()).path.borrow().as_ref().unwrap()).clone()));
    } }
}