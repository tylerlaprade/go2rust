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
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}

fn format_any(value: &dyn Any) -> String {
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
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

pub fn classify(result: Rc<RefCell<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    {
    let _ts_subject = go_recover().clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let recovered = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        drop(_ts_guard);
        { let new_val = (*Rc::new(RefCell::new(Some(format!("int {}", { let __v = (*recovered.borrow().as_ref().unwrap()).clone(); __v })))).borrow().as_ref().unwrap()).clone(); *result.borrow_mut() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let recovered = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        drop(_ts_guard);
        { let new_val = format!("{}{}", "string ".to_string(), (*recovered.borrow().as_ref().unwrap())); *result.borrow_mut() = Some(new_val); };;
    } else {
        let recovered = _ts_subject.clone();
        drop(_ts_guard);
        { let new_val = "unknown".to_string(); *result.borrow_mut() = Some(new_val); };;
    }
    }
}

pub fn call(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut result: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        classify(result_defer_captured.clone());
    }));
        example_com_panicpayload_worker::fail(value.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            if let Err(__go_defer_panic) = go_call_deferred(f, false) {
                std::panic::resume_unwind(__go_defer_panic);
            }
        }
        return result.clone();
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
            result.clone()
        }
    }
}

pub fn dependency_defer_cannot_recover_outer() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut outer: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    let mut callback: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut outer_defer_captured = outer.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        { let new_val = Rc::new(RefCell::new(Some(format!("{}", format!("{}", format_any((go_recover()).borrow().as_ref().unwrap().as_ref())))))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *outer_defer_captured.borrow_mut() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        let mut callback_defer_captured = callback.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        let mut callback_defer_captured_closure_clone = callback_defer_captured.clone(); example_com_panicpayload_worker::run_normal_defer(Rc::new(RefCell::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        if { let __nil_result = (*go_recover().borrow()).is_none(); __nil_result } {
        { let new_val = "nil".to_string(); *callback_defer_captured_closure_clone.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = "stolen".to_string(); *callback_defer_captured_closure_clone.borrow_mut() = Some(new_val); };
    }
        ()
    }) as Box<dyn FnMut() -> ()>))));
        ()
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        { let __go_panic_text = format!("{}", "outer".to_string()); go_panic_any_local(Box::new(__go_panic_text.clone()) as Box<dyn Any>, Some(__go_panic_text), None) };
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
            (outer.clone(), callback.clone())
        }
    }
}

pub fn dependency_intermediate_cannot_recover_outer() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {
    let __go_function_depth_guard = go_enter_function();
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut outer: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    let mut callback: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

    let __go_panic_hook_guard = go_suppress_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut outer_defer_captured = outer.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        { let new_val = Rc::new(RefCell::new(Some(format!("{}", format!("{}", format_any((go_recover()).borrow().as_ref().unwrap().as_ref())))))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *outer_defer_captured.borrow_mut() = __moved_val; };
        ()
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        let mut callback_defer_captured = callback.clone(); __defer_stack.push(Box::new(move || {
        example_com_panicpayload_worker::run(Rc::new(RefCell::new(Some(Box::new(move || {
        let __go_function_depth_guard = go_enter_function();
        if { let __nil_result = (*go_recover().borrow()).is_none(); __nil_result } {
        { let new_val = "nil".to_string(); *callback_defer_captured.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = "stolen".to_string(); *callback_defer_captured.borrow_mut() = Some(new_val); };
    }
        ()
    }) as Box<dyn FnMut() -> ()>))));
    }));
        { let __go_panic_text = format!("{}", "outer".to_string()); go_panic_any_local(Box::new(__go_panic_text.clone()) as Box<dyn Any>, Some(__go_panic_text), None) };
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
            (outer.clone(), callback.clone())
        }
    }
}

fn main() {
    ::example_com_panicpayload_worker::__go_init_all();

    let __go_function_depth_guard = go_enter_function();
    println!("{}", format!("{}", (*call(Rc::new(RefCell::new(Some(Box::new(7i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*call(Rc::new(RefCell::new(Some(Box::new("text".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    let (mut outer, mut callback) = dependency_defer_cannot_recover_outer();
    println!("{} {} {}", format!("{}", "mask".to_string()), format!("{}", { let __v = (*outer.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*callback.borrow().as_ref().unwrap()).clone(); __v }));
    { let (__tmp_0, __tmp_1) = dependency_intermediate_cannot_recover_outer(); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *outer.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *callback.borrow_mut() = __moved_tmp_1; };
    println!("{} {} {}", format!("{}", "direct".to_string()), format!("{}", { let __v = (*outer.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*callback.borrow().as_ref().unwrap()).clone(); __v }));
}