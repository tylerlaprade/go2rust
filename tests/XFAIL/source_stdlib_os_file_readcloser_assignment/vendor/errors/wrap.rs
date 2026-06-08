use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

pub(crate) static errorType: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn internal_reflectlite::r#type::Type + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errorType.lock().unwrap() = None;
    *errorType.lock().unwrap() = Some((*{ let __recv = internal_reflectlite::type_of(Arc::new(Mutex::new(Some({ let __boxed = Box::new(Arc::new(Mutex::new(None::<Option<Box<dyn StdError + Send + Sync>>>))) as Box<dyn Any + Send + Sync>; go_register_any_type_with_elem::<Arc<Mutex<Option<Option<Box<dyn StdError + Send + Sync>>>>>>("pointer", true, "interface", true); __boxed })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).elem(); __result }.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *errorType.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_1() {
    *errorType.lock().unwrap() = Some((*{ let __recv = internal_reflectlite::type_of(Arc::new(Mutex::new(Some({ let __boxed = Box::new(Arc::new(Mutex::new(None::<Option<Box<dyn StdError + Send + Sync>>>))) as Box<dyn Any + Send + Sync>; go_register_any_type_with_elem::<Arc<Mutex<Option<Option<Box<dyn StdError + Send + Sync>>>>>>("pointer", true, "interface", true); __boxed })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).elem(); __result }.lock().unwrap().as_ref().unwrap()).clone());
}


/// Is reports whether any error in err's tree matches target.
///
/// The tree consists of err itself, followed by the errors obtained by repeatedly
/// calling its Unwrap() error or Unwrap() []error method. When err wraps multiple
/// errors, Is examines err followed by a depth-first traversal of its children.
///
/// An error is considered to match a target if it is equal to that target or if
/// it implements a method Is(error) bool such that Is(target) returns true.
///
/// An error type might provide an Is method so it can be treated as equivalent
/// to an existing error. For example, if MyError defines
///
///	func (m MyError) Is(target error) bool { return target == fs.ErrExist }
///
/// then Is(MyError{}, fs.ErrExist) returns true. See [syscall.Errno.Is] for
/// an example in the standard library. An Is method should only shallowly
/// compare err and the target and not call [Unwrap] on either.
pub fn is(err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, target: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> bool {
    if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } || { let __nil_result = (*target.lock().unwrap()).is_none(); __nil_result } {
        return { let __left = err.clone(); let __right = target.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq };
    }

    let mut isComparable = { let __recv = internal_reflectlite::type_of(Arc::new(Mutex::new(Some({ let __err_holder = target.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::r#mod::errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<internal_reflectlite::value::ValueError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).comparable(); __result };
    is_1(err.clone(), target.clone(), Arc::new(Mutex::new(Some(isComparable))))
}

pub fn is_1(mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, target: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, targetComparable: Arc<Mutex<Option<bool>>>) -> bool {
    loop {
        if { let __v = (*targetComparable.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __left = err.clone(); let __right = target.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return true;
    }
        {
        let (mut x, mut ok) = ({
        let val = err.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            (Arc::new(Mutex::new(None::<Box<dyn GoAnonymousInterface1 + Send + Sync>>)), false)
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn GoAnonymousInterface1 + Send + Sync>>)), false)
        }
    });;
        if ok && (*x.lock().unwrap().as_ref().unwrap()).is(target.clone()) {
            return true;;
        }
    }
        {
    let _ts_subject = err.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if false {
        let x: Arc<Mutex<Option<Box<dyn GoAnonymousInterface2 + Send + Sync>>>> = unimplemented!("type info required: type switch on interface case with 0 concrete implementors needs a synthesized trait object");
        drop(_ts_guard);
        { let __rhs_holder = (*x.lock().unwrap().as_ref().unwrap()).unwrap().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        return false;
    };
    } else if false {
        let x: Arc<Mutex<Option<Box<dyn GoAnonymousInterface3 + Send + Sync>>>> = unimplemented!("type info required: type switch on interface case with 0 concrete implementors needs a synthesized trait object");
        drop(_ts_guard);
        { let __range_holder = (*x.lock().unwrap().as_ref().unwrap()).unwrap().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for err in __range_values.iter().cloned() {
        if is_1(err.clone(), target.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = targetComparable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
    } };
        return false;;
    } else {
        let x = _ts_subject.clone();
        drop(_ts_guard);
        return false;;
    }
    }
    }
}

pub trait GoAnonymousInterface1: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool;
    fn is(&self, __arg0: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> bool;
}

impl Clone for Box<dyn GoAnonymousInterface1 + Send + Sync> {
    fn clone(&self) -> Self {
        GoAnonymousInterface1::__go_clone_box_go_anonymous_interface1(self.as_ref())
    }
}

pub trait GoAnonymousInterface2: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface2(&self) -> Box<dyn GoAnonymousInterface2 + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_go_anonymous_interface2(&self, other: &(dyn GoAnonymousInterface2 + Send + Sync)) -> bool;
    fn unwrap(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
}

impl Clone for Box<dyn GoAnonymousInterface2 + Send + Sync> {
    fn clone(&self) -> Self {
        GoAnonymousInterface2::__go_clone_box_go_anonymous_interface2(self.as_ref())
    }
}

pub trait GoAnonymousInterface3: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface3(&self) -> Box<dyn GoAnonymousInterface3 + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_go_anonymous_interface3(&self, other: &(dyn GoAnonymousInterface3 + Send + Sync)) -> bool;
    fn unwrap(&self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>>>>>;
}

impl Clone for Box<dyn GoAnonymousInterface3 + Send + Sync> {
    fn clone(&self) -> Self {
        GoAnonymousInterface3::__go_clone_box_go_anonymous_interface3(self.as_ref())
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
