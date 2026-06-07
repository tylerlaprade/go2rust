use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


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
    } else {
        "<unknown>".to_string()
    }
}

thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

fn main() {
    cmp::__go_init_all();
    container_heap::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_abi::__go_init_all();
    internal_buildcfg::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    internal_filepathlite::__go_init_all();
    internal_godebug::__go_init_all();
    internal_godebugs::__go_init_all();
    internal_goexperiment::__go_init_all();
    internal_gover::__go_init_all();
    internal_goversion::__go_init_all();
    internal_race::__go_init_all();
    internal_stringslite::__go_init_all();
    internal_sync::__go_init_all();
    internal_types_errors::__go_init_all();
    math::__go_init_all();
    math_big::__go_init_all();
    math_bits::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    sort::__go_init_all();
    strconv::__go_init_all();
    strings::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        {
        let mut p = go_recover();;
        if { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "panic:".to_string()), format!("{}", format_any(p.lock().unwrap().as_ref().unwrap().as_ref())));;
        }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        let mut fset = go_token::new_file_set();
        let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File { name: go_ast::new_ident(Arc::new(Mutex::new(Some("main".to_string())))).clone(), decls: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::GenDecl { tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))))), specs: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ValueSpec { names: Arc::new(Mutex::new(Some(vec![go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string()))))]))), r#type: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("int".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Spec + Send + Sync>)))]))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)))]))), ..Default::default() })));
        let (mut pkg, mut err) = { let __recv = Arc::new(Mutex::new(Some(go_types::api::Config::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).check(Arc::new(Mutex::new(Some("main".to_string()))), fset.clone(), Arc::new(Mutex::new(Some(vec![file.clone()]))), Arc::new(Mutex::new(None))); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));
    }
        println!("{} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*{ let __recv = pkg.clone(); let __recv_ptr: *const go_types::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::package::Package }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            ()
        }
    }
}