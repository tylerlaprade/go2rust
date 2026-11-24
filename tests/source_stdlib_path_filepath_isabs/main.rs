#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

/// Source-transpile demo: this fixture opts into transpiling
/// `path/filepath` from the Go stdlib source instead of using the
/// hand-written Rust bridge.
///
/// The pipeline (PackageLoader.shouldTranspileStdlibPackage + vendor crate
/// generation) runs end-to-end, producing vendor/path_filepath/*.rs from
/// the system Go source. Passing this fixture proves IsAbs can come from
/// the source-generated crate; the bridge rows retire when default callers
/// no longer need the external package shim.
fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", path_filepath::is_abs(Arc::new(Mutex::new(Some("/foo".to_string()))))));
    println!("{}", format!("{}", path_filepath::is_abs(Arc::new(Mutex::new(Some("foo".to_string()))))));
}