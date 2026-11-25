#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
fn main() {
    ::context::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    let mut ctx = context::background();
    println!("{}", format!("{}", { let __nil_result = (*(*ctx.lock().unwrap().as_ref().unwrap()).err().lock().unwrap()).is_none(); __nil_result }));
    println!("{}", format!("{}", (*ctx.lock().unwrap().as_ref().unwrap()).done().is_nil()));
}