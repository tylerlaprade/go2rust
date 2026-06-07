use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn accept(err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "ok".to_string()));
    }
}

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_buildcfg::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
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
    strconv::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut err = go_types::api::Error { msg: Arc::new(Mutex::new(Some("boom".to_string()))), ..Default::default() };
    accept(Arc::new(Mutex::new(Some(Box::new(err) as Box<dyn StdError + Send + Sync>))));
}