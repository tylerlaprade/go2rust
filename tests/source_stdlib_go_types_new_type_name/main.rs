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
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut obj = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    let mut basic = { let __seq = { let __seq_holder = go_types::Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[2usize].clone() }.clone();
    println!("{} {} {} {}", format!("{}", (*{ let __recv = obj.clone(); let __recv_ptr: *const go_types::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __recv = basic.clone(); let __recv_ptr: *const go_types::basic::Basic = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::basic::Basic }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())), format!("{}", { let __tmp_x = (*{ let __recv = basic.clone(); let __recv_ptr: *const go_types::basic::Basic = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::basic::Basic }; let __result = unsafe { &*__recv_ptr }.kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_types::basic::BasicKind(Arc::new(Mutex::new(Some(go_types::INT as i32)))); __tmp_x == __tmp_y }), format!("{}", { let __tmp_x = { let __tmp_x = (*{ let __recv = basic.clone(); let __recv_ptr: *const go_types::basic::Basic = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::basic::Basic }; let __result = unsafe { &*__recv_ptr }.info(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_types::basic::BasicInfo(Arc::new(Mutex::new(Some(go_types::IS_INTEGER as i32)))); __tmp_x & __tmp_y }; let __tmp_y = go_types::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y }));
}