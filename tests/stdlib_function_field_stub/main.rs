use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_buildcfg::__go_init_all();
    internal_filepathlite::__go_init_all();
    internal_godebug::__go_init_all();
    internal_goexperiment::__go_init_all();
    internal_gover::__go_init_all();
    internal_goversion::__go_init_all();
    internal_stringslite::__go_init_all();
    internal_types_errors::__go_init_all();
    math::__go_init_all();
    math_big::__go_init_all();
    math_bits::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let _ = go_types::api::Config { error: Arc::new(Mutex::new(Some(Box::new(move |err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>| {
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync>))), ..Default::default() };
    println!("{}", format!("{}", "ok".to_string()));
}