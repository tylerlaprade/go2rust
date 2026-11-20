#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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
    ::internal_race::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_sync::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        {
        let mut p = go_recover();;
        if { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "panic:".to_string()), format!("{}", { let __text = format_any(p.lock().unwrap().as_ref().unwrap().as_ref()); __text }));;
        }
    }
        ()
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        let mut fset = go_token::new_file_set();
        let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File {
            name: go_ast::new_ident(Arc::new(Mutex::new(Some("main".to_string())))).clone(),
            decls: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::GenDecl { tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i64))))))), specs: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ValueSpec { names: Arc::new(Mutex::new(Some(vec![go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string()))))]))), r#type: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("int".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Spec + Send + Sync>)))]))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)))]))),
            ..Default::default()
        })));
        let (mut pkg, mut err) = {
            let __recv = Arc::new(Mutex::new(Some(go_types::api::Config::default())));
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).check(
                Arc::new(Mutex::new(Some("main".to_string()))),
                fset.clone(),
                Arc::new(Mutex::new(Some(vec![file.clone()]))),
                Arc::new(Mutex::new(None)),
            );
            __result
        };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
        println!("{} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*{ let __recv = pkg.clone(); let __recv_ptr: *const go_types::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::package::Package }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                std::panic::resume_unwind(__go_defer_panic);
            }
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
            ()
        }
    }
}