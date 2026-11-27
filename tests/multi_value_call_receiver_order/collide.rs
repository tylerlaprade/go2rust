use crate::{
    GoSlice,
    GoSliceAppendBoxed,
    GoSliceFromSlice,
    GoSliceHeader,
    Vec,
    go_recover,
    go_restore_panic_hook,
    go_resume_unrecovered_panic,
    go_runtime_error_text,
    go_silence_panic_hook,
    go_store_panic_payload,
};

use crate::{divmod};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn collide() {
    let mut __go_mv_9_0 = Rc::new(RefCell::new(Some("user".to_string())));
    let (mut __go_mv_1_0, mut __go_mv_1_1) = divmod(Rc::new(RefCell::new(Some(9))), Rc::new(RefCell::new(Some(4))));
    println!("{} {}", format!("{}", __go_mv_1_0), format!("{}", __go_mv_1_1));
    println!("{}", format!("{}", { let __v = (*__go_mv_9_0.borrow().as_ref().unwrap()).clone(); __v }));
}