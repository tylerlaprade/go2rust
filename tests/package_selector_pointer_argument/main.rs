#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct reader {
    pub current: Rc<RefCell<Option<example_com_package_selector_pointer_argument_helper::r#mod::Pkg>>>,
}

impl reader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.current.clone();
        Self {
            current: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.current.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl reader {
    pub fn run(&self) {
        example_com_package_selector_pointer_argument_helper::r#use({ let __field = self.current.clone(); __field });
    }
}

fn main() {
    ::example_com_package_selector_pointer_argument_helper::__go_init_all();

    let mut p = Rc::new(RefCell::new(Some(example_com_package_selector_pointer_argument_helper::r#mod::Pkg { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut r = Rc::new(RefCell::new(Some(reader { current: p.clone(), ..Default::default() })));
    (*r.borrow().as_ref().unwrap()).run();
}

impl GoValueClone for reader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
