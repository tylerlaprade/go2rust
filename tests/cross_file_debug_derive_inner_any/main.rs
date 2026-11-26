#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod aaa_outer;
mod bbb_inner;
use aaa_outer::*;
use bbb_inner::*;

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
fn main() {
    go_install_panic_exit();
    let mut r#in = Rc::new(RefCell::new(Some(Inner { tag: Rc::new(RefCell::new(Some("k".to_string()))), data: Rc::new(RefCell::new(Some(Box::new(42i64) as Box<dyn Any>))), ..Default::default() })));
    let mut out = Rc::new(RefCell::new(Some(Outer { name: Rc::new(RefCell::new(Some("n".to_string()))), inner: r#in.clone(), ..Default::default() })));
    print!("{} {} {}\n", (*(*out.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*(*out.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).clone(), match (*(*out.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).data.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() });
}