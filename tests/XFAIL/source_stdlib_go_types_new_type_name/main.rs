use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    go_types::__go_init_all();
    internal_types_errors::__go_init_all();

    let mut obj = go_types::new_type_name(Arc::new(Mutex::new(Some(token::NO_POS))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    println!("{}", format!("{}", (*{ let __recv = obj.clone(); let __recv_ptr: *const go_types::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));
}