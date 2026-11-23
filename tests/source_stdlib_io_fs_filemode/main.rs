#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::io_fs::__go_init_all();

    let mut mode = Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(((io_fs::MODE_DIR as u32 | io_fs::MODE_SETUID as u32) as u32 | 0o640 as u32) as u32)))))));
    println!("{}", format!("{}", {
        let __tmp_x = (*io_fs::r#mod::FileMode::r#type(&(*mode.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(io_fs::MODE_DIR as u32))));
        __tmp_x == __tmp_y
    }));
    println!("{}", format!("{}", (*io_fs::r#mod::FileMode::string(&(*mode.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())));
}