use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn make_qualifier() -> go_types::typestring::Qualifier {
    return Arc::new(Mutex::new(Some(Box::new(move |pkg: Arc<Mutex<Option<go_types::package::Package>>>| -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("".to_string())))
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_types::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
}

pub fn use_qualifier(qualifier: go_types::typestring::Qualifier) -> Arc<Mutex<Option<String>>> {
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_types::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = qualifier.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_types::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None))) }
}

pub fn forward_qualifier(qualifier: go_types::typestring::Qualifier) -> Arc<Mutex<Option<String>>> {
    use_qualifier(qualifier.clone())
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

    println!("{}", format!("{}", format!("{}{}", "qualifier:".to_string(), (*forward_qualifier(make_qualifier()).lock().unwrap().as_ref().unwrap()))));
}