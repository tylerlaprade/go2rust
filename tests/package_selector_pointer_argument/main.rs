use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct reader {
    pub current: Rc<RefCell<Option<example_com_package_selector_pointer_argument_helper::Pkg>>>,
}

impl reader {
    pub fn __go_value_clone(&self) -> Self {
        Self { current: self.current.clone() }
    }
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.current.borrow().as_ref().unwrap()))
    }
}


impl reader {
    pub fn run(&self) {
        example_com_package_selector_pointer_argument_helper::r#use(self.current.clone());
    }
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(example_com_package_selector_pointer_argument_helper::Pkg { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut r = Rc::new(RefCell::new(Some(reader { current: p.clone(), ..Default::default() })));
    (*r.borrow_mut().as_mut().unwrap()).run();
}