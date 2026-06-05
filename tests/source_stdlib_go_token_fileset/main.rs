use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    cmp::__go_init_all();
    go_token::__go_init_all();
    internal_abi::__go_init_all();
    internal_race::__go_init_all();
    internal_sync::__go_init_all();
    slices::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();

    let mut src = Arc::new(Mutex::new(Some("package p\nvar x int\n".to_string())));
    let mut fset = go_token::new_file_set();
    let mut file = { let __recv = fset.clone(); let __recv_ptr: *mut go_token::position::FileSet = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut go_token::position::FileSet }; let __result = unsafe { &mut *__recv_ptr }.add_file(Arc::new(Mutex::new(Some("p.go".to_string()))), Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some((*src.lock().unwrap().as_ref().unwrap()).len() as i32)))); __result };
    { let __recv = file.clone(); let __recv_ptr: *mut go_token::position::File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut go_token::position::File }; let __result = unsafe { &mut *__recv_ptr }.set_lines_for_content(Arc::new(Mutex::new(Some(({ let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec())))); __result };
    let mut pos = { let __recv = file.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.pos(Arc::new(Mutex::new(Some(11)))); __result };
    let mut position = { let __recv = fset.clone(); let __recv_ptr: *const go_token::position::FileSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::FileSet }; let __result = unsafe { &*__recv_ptr }.position(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    println!("{} {} {} {} {}", format!("{}", { let __tmp_x = { let __recv = fset.clone(); let __recv_ptr: *const go_token::position::FileSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::FileSet }; let __result = unsafe { &*__recv_ptr }.base(); __result }; let __tmp_y = { let __recv = file.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.base(); __result }; __tmp_x > __tmp_y }), format!("{}", (*{ let __recv = file.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*position.lock().unwrap().as_ref().unwrap()).filename.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __tmp_x = (*{ let __field = (*position.lock().unwrap().as_ref().unwrap()).line.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y }), format!("{}", { let __tmp_x = (*{ let __field = (*position.lock().unwrap().as_ref().unwrap()).column.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y }));
}