#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::bytes::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::os_exec::__go_init_all();
    ::path_filepath::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::time::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut path, mut err) = os_exec::look_path(Arc::new(Mutex::new(Some("go".to_string()))));
    println!("{} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __tmp_x = ({ let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = "".to_string(); __tmp_x != __tmp_y }));
}