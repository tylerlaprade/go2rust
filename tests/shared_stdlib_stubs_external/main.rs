#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
fn main() {
    ::example_com_sharedstubmain_aliases::__go_init_all();
    ::example_com_sharedstubmain_typeparams::__go_init_all();

    {
            let __go_print_arg_0 = format!("{}", example_com_sharedstubmain_typeparams::count());
            eprintln!("{}", __go_print_arg_0)
        };
}