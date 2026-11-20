#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub fn classify(result: Rc<RefCell<Option<String>>>) {
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

fn main() {
    ::example_com_panicpayload_worker::__go_init_all();

    println!("{}", format!("{}", (*call(Rc::new(RefCell::new(Some(Box::new(7i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*call(Rc::new(RefCell::new(Some(Box::new("text".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
}