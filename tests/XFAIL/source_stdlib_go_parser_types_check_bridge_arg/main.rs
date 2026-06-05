use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_parser::__go_init_all();
    go_scanner::__go_init_all();
    go_token::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    internal_filepathlite::__go_init_all();
    internal_stringslite::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    strings::__go_init_all();

    let mut fset = go_token::new_file_set();
    let (mut file, mut err) = go_parser::parse_file(fset.clone(), Arc::new(Mutex::new(Some("input.go".to_string()))), Arc::new(Mutex::new(Some(Box::new("package main\nvar x int\n".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(0 as u64))))))));
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "parse failed".to_string()));
        return;
    }
    let (mut pkg, __tmp_1) = { let __recv = Arc::new(Mutex::new(Some(types_Config::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).check("main".to_string(), fset.clone(), Arc::new(Mutex::new(Some(vec![file.clone()]))), ()); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    println!("{} {}", format!("{}", (*err.lock().unwrap()).is_none()), format!("{}", (*{ let __recv = pkg.clone(); let __recv_ptr: *mut types_Package = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut types_Package }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));
}