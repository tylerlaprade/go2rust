use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::sync::{Arc, Mutex};


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

pub fn named_type() -> Arc<Mutex<Option<go_types::named::Named>>> {
    let mut obj = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    return go_types::new_named(obj.clone(), Arc::new(Mutex::new(Some(Box::new(go_types::basic::BasicPtr({ let __seq = { let __seq_holder = go_types::Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[2usize].clone() }.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), Arc::new(Mutex::new(None)));
}

pub fn forms(named: Arc<Mutex<Option<go_types::named::Named>>>) -> i32 {
    if { let __nil_result = (*named.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }
    let mut count = Arc::new(Mutex::new(Some(0)));
    for recv in &vec![Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(go_types::pointer::PointerPtr(go_types::new_pointer(Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))] {
        if { let __nil_result = (*recv.lock().unwrap()).is_some(); __nil_result } {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    return { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn make_type() -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(go_types::pointer::PointerPtr(go_types::new_pointer(Arc::new(Mutex::new(Some(Box::new(go_types::named::NamedPtr(named_type().clone())) as Box<dyn go_types::r#type::Type + Send + Sync>)))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res.clone();
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
            res.clone()
        }
    }
}

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_buildcfg::__go_init_all();
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
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    println!("{}", format!("{}", forms(named_type())));
    println!("{}", format!("{}", { let __nil_result = (*make_type().lock().unwrap()).is_some(); __nil_result }));
}