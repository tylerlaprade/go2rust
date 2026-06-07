use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

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

    let mut terms = Arc::new(Mutex::new(Some(vec![go_types::new_term(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(None)))])));
    { let __recv = { let __seq = { let __seq_holder = terms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result };
    { let __range_holder = terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for term in __range_values.iter() {
        { let __recv = term.clone(); let __recv_ptr: *const go_types::union::Term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::union::Term }; let __result = unsafe { &*__recv_ptr }.r#type(); __result };
    } }
    println!("{}", format!("{}", "ok".to_string()));
}