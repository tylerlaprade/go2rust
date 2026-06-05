use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn forms(named: Arc<Mutex<Option<go_types::named::Named>>>) -> i32 {
    if (*named.lock().unwrap()).is_none() {
        return 0;
    }
    let mut count = Arc::new(Mutex::new(Some(0)));
    for recv in &vec![Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(go_types::pointer::PointerPtr(go_types::new_pointer(Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))] {
        if (*recv.lock().unwrap()).is_some() {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    return { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v };
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

    println!("{}", format!("{}", "ok".to_string()));
}