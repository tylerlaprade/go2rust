use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use crate::{divmod};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn other() {
    let (mut x, mut y) = divmod(Rc::new(RefCell::new(Some(9))), Rc::new(RefCell::new(Some(4))));
    println!("{} {}", format!("{}", x), format!("{}", y));
    let (mut __go_mv_1_0, mut __go_mv_1_1) = divmod(Rc::new(RefCell::new(Some(7))), Rc::new(RefCell::new(Some(2))));
    println!("{} {}", format!("{}", __go_mv_1_0), format!("{}", __go_mv_1_1));
}