use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

/// Source-transpile demo: this fixture opts into transpiling
/// `path/filepath` from the Go stdlib source instead of using the
/// hand-written Rust bridge.
///
/// The pipeline (PackageLoader.shouldTranspileStdlibPackage + vendor crate
/// generation) runs end-to-end, producing vendor/path_filepath/*.rs from
/// the system Go source. Today the resulting Rust does not yet compile
/// because of multiple transpiler gaps that the full path/filepath
/// surface exposes (wrapped-type arithmetic, generics handling, etc.).
/// Each is a focused fixture target in its own right.
///
/// When those gaps close, this fixture promotes from XFAIL to passing
/// (./test.sh auto-promotes) and the matching filepath-* rows in
/// docs/bridge_debt.md become retirable.
fn main() {
    internal_filepathlite::__go_init_all();
    internal_stringslite::__go_init_all();
    path_filepath::__go_init_all();

    println!("{}", format!("{}", path_filepath::is_abs(Arc::new(Mutex::new(Some("/foo".to_string()))))));
    println!("{}", format!("{}", path_filepath::is_abs(Arc::new(Mutex::new(Some("foo".to_string()))))));
}