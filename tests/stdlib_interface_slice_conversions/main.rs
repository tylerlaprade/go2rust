#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn named_type() -> Arc<Mutex<Option<go_types::named::Named>>> {
    let mut obj = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i64))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    return go_types::new_named(obj.clone(), Arc::new(Mutex::new(Some(Box::new(go_types::basic::BasicPtr({ let __seq = { let __seq_holder = go_types::Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[2usize].clone() }.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), Arc::new(Mutex::new(None)));
}

pub fn forms(named: Arc<Mutex<Option<go_types::named::Named>>>) -> i64 {
    if { let __nil_result = (*named.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    let mut count = Arc::new(Mutex::new(Some(0i64)));
    for recv in &vec![Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(go_types::pointer::PointerPtr(go_types::new_pointer(Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))] {
        if { let __nil_result = (*recv.lock().unwrap()).is_some(); __nil_result } {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    return { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn make_type() -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut res: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(go_types::pointer::PointerPtr(go_types::new_pointer(Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named_type().clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                std::panic::resume_unwind(__go_defer_panic);
            }
        }
        return res.clone();
    }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            drop(__go_panic_hook_guard);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = go_call_deferred(f, true) {
                    go_replace_panic_payload(__go_defer_panic);
                }
            }
            drop(__go_panic_hook_guard);
            go_resume_unrecovered_panic();
            res.clone()
        }
    }
}

fn main() {
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::container_heap::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_constant::__go_init_all();
    ::go_token::__go_init_all();
    ::go_types::__go_init_all();
    ::go_version::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_buildcfg::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_gover::__go_init_all();
    ::internal_goversion::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", forms(named_type())));
    println!("{}", format!("{}", { let __nil_result = (*make_type().lock().unwrap()).is_some(); __nil_result }));
}