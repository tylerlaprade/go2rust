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


fn go_install_panic_exit() {
    std::panic::set_hook(Box::new(|__info| {
        let __text = go_panic_payload_text(__info.payload());
        {
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        eprintln!("panic: {}", __text);
        std::process::exit(2);
    }));
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text(__text.clone());
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

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
    go_install_panic_exit();
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
