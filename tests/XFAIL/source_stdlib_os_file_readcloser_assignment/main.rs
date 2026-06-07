use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn main() {
    ::cmp::__go_init_all();
    ::errors::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_asan::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_byteorder::__go_init_all();
    ::internal_chacha8rand::__go_init_all();
    ::internal_coverage_rtcov::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_goarch::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_goos::__go_init_all();
    ::internal_itoa::__go_init_all();
    ::internal_msan::__go_init_all();
    ::internal_oserror::__go_init_all();
    ::internal_poll::__go_init_all();
    ::internal_profilerecord::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_reflectlite::__go_init_all();
    ::internal_runtime_atomic::__go_init_all();
    ::internal_runtime_exithook::__go_init_all();
    ::internal_runtime_maps::__go_init_all();
    ::internal_runtime_math::__go_init_all();
    ::internal_runtime_sys::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_sync::__go_init_all();
    ::internal_syscall_execenv::__go_init_all();
    ::internal_syscall_unix::__go_init_all();
    ::internal_testlog::__go_init_all();
    ::internal_unsafeheader::__go_init_all();
    ::io::__go_init_all();
    ::io_fs::__go_init_all();
    ::iter::__go_init_all();
    ::math_bits::__go_init_all();
    ::os::__go_init_all();
    ::path::__go_init_all();
    ::runtime::__go_init_all();
    ::slices::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::syscall::__go_init_all();
    ::time::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let (mut file, mut err) = os::open(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = os::Args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<io_fs::r#mod::PathError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os::error::SyscallError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os::file::LinkError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<syscall::syscall_unix::Errno>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
    }
    let mut rc: Arc<Mutex<Option<Box<dyn io::r#mod::ReadCloser + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(os::types::FilePtr(file.clone())) as Box<dyn io::r#mod::ReadCloser + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *rc.lock().unwrap() = __iface_value; };
    {
        let mut err = (*rc.lock().unwrap().as_mut().unwrap()).close();;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            std::panic::panic_any({ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<io_fs::r#mod::PathError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os::error::SyscallError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<os::file::LinkError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<syscall::syscall_unix::Errno>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });;
        }
    }
    eprintln!("{}", format!("{}", "closed".to_string()));
}