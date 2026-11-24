#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

fn main() {
    ::internal_bytealg::__go_init_all();
    ::io::__go_init_all();
    ::io_fs::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut info, mut err) = os::stat(".".to_string());
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any + Send + Sync>, Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } });
    }
    println!("{}", format!("{}", {
        let __tmp_x = (*io_fs::r#mod::FileMode::r#type(&(*(*info.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(io_fs::MODE_DIR as u32))));
        __tmp_x == __tmp_y
    }));
}