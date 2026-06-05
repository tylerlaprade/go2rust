use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn pass_tuple(t: Arc<Mutex<Option<go_types::tuple::Tuple>>>) -> bool {
    has(Arc::new(Mutex::new(Some(Box::new(go_types::tuple::TuplePtr(t.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))))
}

pub fn has(t: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>) -> bool {
    true
}

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

    let mut tuple = go_types::new_tuple(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))]))));
    println!("{}", format!("{}", pass_tuple(tuple.clone())));
}