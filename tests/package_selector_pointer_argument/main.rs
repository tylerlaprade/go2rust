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

impl GoJsonDecode for reader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl reader {
    pub fn run(&self) {
        example_com_package_selector_pointer_argument_helper::r#use({ let __field = self.current.clone(); __field });
    }
}

fn main() {
    example_com_package_selector_pointer_argument_helper::__go_init_all();

    let mut p = Rc::new(RefCell::new(Some(example_com_package_selector_pointer_argument_helper::Pkg { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut r = Rc::new(RefCell::new(Some(reader { current: p.clone(), ..Default::default() })));
    (*r.borrow().as_ref().unwrap()).run();
}