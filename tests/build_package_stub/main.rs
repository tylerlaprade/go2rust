use go2rust_stdlib_stubs::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::bytes::__go_init_all();
    ::go_build::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_sync::__go_init_all();
    ::path_filepath::__go_init_all();
    ::strings::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut pkg, mut err) = (*go_build::Default.lock().unwrap().as_ref().unwrap()).import(Arc::new(Mutex::new(Some("fmt".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(go_build::r#mod::ImportMode(Arc::new(Mutex::new(Some(go_build::FIND_ONLY as u64))))))));
    println!("{} {} {} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).goroot.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).import_path.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __tmp_x = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y }));
    println!("{}", format!("{}", { let __tmp_x = { let __selector_holder = (*go_build::Default.lock().unwrap().as_ref().unwrap()).g_o_r_o_o_t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y }));
    println!("{} {}", format!("{}", go_build::is_local_import(Arc::new(Mutex::new(Some("./pkg".to_string()))))), format!("{}", go_build::is_local_import(Arc::new(Mutex::new(Some("fmt".to_string()))))));
}