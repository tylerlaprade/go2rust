include!("__go2rust_helpers.rs");
mod aaa_outer;
mod bbb_inner;
use aaa_outer::*;
use bbb_inner::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut r#in = Rc::new(RefCell::new(Some(Inner { tag: Rc::new(RefCell::new(Some("k".to_string()))), data: Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>))), ..Default::default() })));
    let mut out = Rc::new(RefCell::new(Some(Outer { name: Rc::new(RefCell::new(Some("n".to_string()))), inner: r#in.clone(), ..Default::default() })));
    print!("{} {} {}\n", (*(*out.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*(*out.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).clone(), format_any((*(*out.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).data.borrow().as_ref().unwrap().as_ref()));
}