#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod detail;
use detail::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct valueError {
    pub code: Arc<Mutex<Option<String>>>,
}

impl valueError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for valueError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for valueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for valueError {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.code.lock().unwrap().as_ref().cloned() }; let __right = { other.code.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pairError {
    pub a: Arc<Mutex<Option<i64>>>,
    pub b: Arc<Mutex<Option<i64>>>,
}

impl pairError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_1_0,
        }
    }
}


impl Default for pairError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            a: __go_default_0_0,
            b: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pairError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for pairError {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.a.lock().unwrap().as_ref().cloned() }; let __right = { other.a.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.b.lock().unwrap().as_ref().cloned() }; let __right = { other.b.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct nestedError {
    pub key: Arc<Mutex<Option<nestedCode>>>,
}

impl nestedError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            key: __go_clone_0_0,
        }
    }
}


impl Default for nestedError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(nestedCode::default())));
        Self {
            key: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for nestedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for nestedError {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.key.lock().unwrap().as_ref().cloned() }; let __right = { other.key.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct nanError {
    pub value: Arc<Mutex<Option<f64>>>,
}

impl nanError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for nanError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for nanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for nanError {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.value.lock().unwrap().as_ref().cloned() }; let __right = { other.value.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


pub trait namedValue: std::fmt::Display + Any {
    fn __go_clone_box_named_value(&self) -> Box<dyn namedValue + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_named_value(&self, other: &(dyn namedValue + Send + Sync)) -> bool;
    fn value(&self) -> f64;
}

impl Clone for Box<dyn namedValue + Send + Sync> {
    fn clone(&self) -> Self {
        namedValue::__go_clone_box_named_value(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct namedNaN {
    pub value: Arc<Mutex<Option<f64>>>,
}

impl namedNaN {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for namedNaN {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for namedNaN {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for namedNaN {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.value.lock().unwrap().as_ref().cloned() }; let __right = { other.value.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone, Default)]
pub struct namedHolder {
    pub value: Arc<Mutex<Option<Box<dyn namedValue + Send + Sync>>>>,
}

impl namedHolder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for namedHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for namedHolder {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.value.lock().unwrap().clone() }; let __right = { other.value.lock().unwrap().clone() }; match (__left.as_ref(), __right.as_ref()) { (Some(__left), Some(__right)) => __left.as_ref().__go_eq_named_value(__right.as_ref()), (None, None) => true, _ => false } }
        )
    }
}


#[derive(Debug, Clone, Default)]
pub struct pointerFieldError {
    pub p: Arc<Mutex<Option<i64>>>,
}

impl pointerFieldError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.p.clone();
        Self {
            p: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for pointerFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for pointerFieldError {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left_some = self.p.lock().unwrap().is_some(); let __right_some = other.p.lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Arc::ptr_eq(&self.p, &other.p)) }
        )
    }
}


#[derive(Debug, Clone, Default)]
pub struct nilSafeError {
}

impl nilSafeError {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for nilSafeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct cycleError {
    pub next: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>,
}

impl cycleError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.next.clone();
        Self {
            next: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for cycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for cycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


pub static ErrBusy: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<pointerError>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrBusy.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *ErrBusy.lock().unwrap() = Some(Arc::new(Mutex::new(Some(pointerError { n: Arc::new(Mutex::new(Some(1))), ..Default::default() }))));
}


pub(crate) fn __go_zero_globals() {
    *ErrBusy.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_0() {
    *ErrBusy.lock().unwrap() = Some(Arc::new(Mutex::new(Some(pointerError { n: Arc::new(Mutex::new(Some(1))), ..Default::default() }))));
}


impl valueError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for valueError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_valueError(pub Arc<Mutex<Option<valueError>>>);

impl std::fmt::Display for __go_error_ptr_valueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_valueError {}


impl pointerFieldError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("pointer field".to_string())))
    }
}

impl StdError for pointerFieldError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_pointerFieldError(pub Arc<Mutex<Option<pointerFieldError>>>);

impl std::fmt::Display for __go_error_ptr_pointerFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_pointerFieldError {}


impl nilSafeError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("custom".to_string())))
    }

    pub fn unwrap(&self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("inner".to_string())))))
    }
}

impl StdError for nilSafeError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_nilSafeError(pub Arc<Mutex<Option<nilSafeError>>>);

impl std::fmt::Display for __go_error_ptr_nilSafeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.lock().unwrap().is_none();
        if !__go_is_nil {
            let __guard = self.0.lock().unwrap();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_nil_safe_error_error(Arc::new(Mutex::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.lock().unwrap(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_nilSafeError {}


impl cycleError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("cycle".to_string())))
    }

    pub fn unwrap(&self) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        { let __err_source = { let __field = self.next.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }
    }
}

impl StdError for cycleError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_cycleError(pub Arc<Mutex<Option<cycleError>>>);

impl std::fmt::Display for __go_error_ptr_cycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.lock().unwrap().is_none();
        if !__go_is_nil {
            let __guard = self.0.lock().unwrap();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_cycle_error_error(Arc::new(Mutex::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.lock().unwrap(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_cycleError {}


impl pairError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{}:{}", format!("{}", (*self.a.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.b.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl StdError for pairError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_pairError(pub Arc<Mutex<Option<pairError>>>);

impl std::fmt::Display for __go_error_ptr_pairError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_pairError {}


impl nestedError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = (*self.key.lock().unwrap().as_ref().unwrap()).code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for nestedError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_nestedError(pub Arc<Mutex<Option<nestedError>>>);

impl std::fmt::Display for __go_error_ptr_nestedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_nestedError {}


impl nanError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("nan".to_string())))
    }
}

impl StdError for nanError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_nanError(pub Arc<Mutex<Option<nanError>>>);

impl std::fmt::Display for __go_error_ptr_nanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_nanError {}


impl namedNaN {
    pub fn value(&self) -> f64 {
        return (*self.value.lock().unwrap().as_ref().unwrap());
    }
}

impl namedValue for namedNaN {
    fn value(&self) -> f64 {
        namedNaN::value(self)
    }
    fn __go_clone_box_named_value(&self) -> Box<dyn namedValue + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namedValue + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named_value(&self, other: &(dyn namedValue + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedNaN>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct namedNaNPtr(pub Arc<Mutex<Option<namedNaN>>>);

impl std::fmt::Display for namedNaNPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namedValue for namedNaNPtr {
    fn value(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        namedNaN::value(__recv)
    }
    fn __go_clone_box_named_value(&self) -> Box<dyn namedValue + Send + Sync> {
        Box::new(self.clone()) as Box<dyn namedValue + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named_value(&self, other: &(dyn namedValue + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedNaNPtr>() {
            let __go_left_nil = { self.0.lock().unwrap().is_none() };
            let __go_right_nil = { __other.0.lock().unwrap().is_none() };
            if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&self.0, &__other.0) }
        } else {
            false
        }
    }
}

impl pointerError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("pointer:{}", (*self.n.lock().unwrap().as_ref().unwrap())))))
    }
}

impl StdError for pointerError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_pointerError(pub Arc<Mutex<Option<pointerError>>>);

impl std::fmt::Display for __go_error_ptr_pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.lock().unwrap().is_none();
        if !__go_is_nil {
            let __guard = self.0.lock().unwrap();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_error_result = go_silent_catch_unwind(|| __go_nil_recv_pointer_error_error(Arc::new(Mutex::new(None))));
            match __go_error_result { Ok(__text) => { let __guard = __text.lock().unwrap(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_pointerError {}


pub fn return_busy() -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_pointerError({ let __arg_holder = ErrBusy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Arc<dyn StdError + Send + Sync>)))
}

pub fn same_by_switch(err: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>, target: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) -> bool {
    { let _switch_val = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    if { let __left_holder = _switch_val.clone(); let __left_value = { (*__left_holder.lock().unwrap()).clone() }; let __right_holder = { let __err_source = target.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __right_value = { (*__right_holder.lock().unwrap()).clone() }; match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__left), Some(__right)) => if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_cycleError>(), __right.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<nanError>(), __right.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_nanError>(), __right.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<nestedError>(), __right.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_nestedError>(), __right.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_nilSafeError>(), __right.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<pairError>(), __right.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_pairError>(), __right.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_pointerError>(), __right.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<pointerFieldError>(), __right.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_pointerFieldError>(), __right.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<valueError>(), __right.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_valueError>(), __right.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__left, &**__right) { true } else { false }, (None, None) => true, _ => false } } {
            return true;
        }
    }
    false
}

fn main() {
    __go_init_all();
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut left: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(valueError { code: Arc::new(Mutex::new(Some("same".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut equal: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(valueError { code: Arc::new(Mutex::new(Some("same".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut different: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(valueError { code: Arc::new(Mutex::new(Some("different".to_string()))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    println!("{} {}", format!("{}", { let __left = left.clone(); let __right = equal.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = left.clone(); let __right = different.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut pointer = Arc::new(Mutex::new(Some(pointerError { n: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    let mut first: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_pointerError(pointer.clone())) as Arc<dyn StdError + Send + Sync>)));
    let mut second: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_pointerError(pointer.clone())) as Arc<dyn StdError + Send + Sync>)));
    println!("{}", format!("{}", { let __left = first.clone(); let __right = second.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));
    { let new_val = 2; *(*pointer.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", first.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", second.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));

    println!("{}", format!("{}", { let __left = return_busy().clone(); let __right = Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_pointerError({ let __arg_holder = ErrBusy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Arc<dyn StdError + Send + Sync>))); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));
    println!("{} {}", format!("{}", same_by_switch({ let __err_source = left.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, { let __err_source = equal.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) })), format!("{}", same_by_switch({ let __err_source = first.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }, { let __err_source = second.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) })));

    let mut pairLeft: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(pairError { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut pairRight: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(pairError { a: Arc::new(Mutex::new(Some(1))), b: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut compared = GoChannel::<bool>::new();
    let compared_thread = compared.clone(); let pairLeft_thread = pairLeft.clone(); let pairRight_thread = pairRight.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1000; __tmp_x < __tmp_y } {
        let _ = { let __left = pairLeft_thread.clone(); let __right = pairRight_thread.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        compared_thread.send(true);;;
    });
    let compared_thread = compared.clone(); let pairLeft_thread = pairLeft.clone(); let pairRight_thread = pairRight.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1000; __tmp_x < __tmp_y } {
        let _ = { let __left = pairRight_thread.clone(); let __right = pairLeft_thread.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        compared_thread.send(true);;;
    });
    println!("{}", format!("{}", compared.recv().unwrap_or_default() && compared.recv().unwrap_or_default()));

    let mut nestedLeft: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(nestedError { key: Arc::new(Mutex::new(Some(nestedCode { code: Arc::new(Mutex::new(Some("nested".to_string()))), ..Default::default() }))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut nestedRight: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(nestedError { key: Arc::new(Mutex::new(Some(nestedCode { code: Arc::new(Mutex::new(Some("nested".to_string()))), ..Default::default() }))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    println!("{}", format!("{}", { let __left = nestedLeft.clone(); let __right = nestedRight.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut zero = Arc::new(Mutex::new(Some(0.0)));
    let mut nan: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(nanError { value: Arc::new(Mutex::new(Some({ let __bin_zero = ({ let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_zero / __bin_zero }))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut nanCopy = { let __err_source = nan.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    println!("{} {}", format!("{}", { let __left = nan.clone(); let __right = nan.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = nan.clone(); let __right = nanCopy.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut holder = Arc::new(Mutex::new(Some(namedHolder { value: Arc::new(Mutex::new(Some(Box::new(namedNaN { value: Arc::new(Mutex::new(Some({ let __bin_zero = ({ let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_zero / __bin_zero }))), ..Default::default() }) as Box<dyn namedValue + Send + Sync>))), ..Default::default() })));
    let mut holderCopy = { let __owned = holder.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    println!("{} {}", format!("{}", { let __bin_holder = ({ let __v = (*holder.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); __bin_holder == __bin_holder }), format!("{}", { let __tmp_x = ({ let __v = (*holder.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); let __tmp_y = ({ let __v = (*holderCopy.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); __tmp_x == __tmp_y }));

    let mut nilNamedLeft: Arc<Mutex<Option<namedNaN>>> = Arc::new(Mutex::new(None));
    let mut nilNamedRight: Arc<Mutex<Option<namedNaN>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", { let __tmp_x = namedHolder { value: Arc::new(Mutex::new(Some(Box::new(namedNaNPtr(nilNamedLeft.clone())) as Box<dyn namedValue + Send + Sync>))), ..Default::default() }; let __tmp_y = namedHolder { value: Arc::new(Mutex::new(Some(Box::new(namedNaNPtr(nilNamedRight.clone())) as Box<dyn namedValue + Send + Sync>))), ..Default::default() }; __tmp_x == __tmp_y }));

    let (mut firstNumber, mut secondNumber) = (Arc::new(Mutex::new(Some(1i64))), Arc::new(Mutex::new(Some(1i64))));
    let mut pointerFieldLeft: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(pointerFieldError { p: firstNumber.clone().clone(), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut pointerFieldRight: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(pointerFieldError { p: secondNumber.clone().clone(), ..Default::default() }) as Arc<dyn StdError + Send + Sync>)));
    let mut pointerFieldCopy = { let __err_source = pointerFieldLeft.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    println!("{} {}", format!("{}", { let __left = pointerFieldLeft.clone(); let __right = pointerFieldRight.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = pointerFieldLeft.clone(); let __right = pointerFieldCopy.clone(); let __left_value = { (*__left.lock().unwrap()).clone() }; let __right_value = { (*__right.lock().unwrap()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pairError>(), __r.downcast_ref::<pairError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pairError>(), __r.downcast_ref::<__go_error_ptr_pairError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError>(), __r.downcast_ref::<__go_error_ptr_pointerError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.lock().unwrap().is_none() }; let __go_right_nil = { __go_right.0.lock().unwrap().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Arc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut nilSafe: Arc<Mutex<Option<nilSafeError>>> = Arc::new(Mutex::new(None));
    let mut nilSafeAsError: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_nilSafeError(nilSafe.clone())) as Arc<dyn StdError + Send + Sync>)));
    println!("{} {}", format!("{}", { let __go_print_value = { let __go_print_handle = &(nilSafeAsError); let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __go_print_value = { let __go_print_handle = &(Arc::new(Mutex::new(go_errors_unwrap({ let __err_handle = { let __err_source = nilSafeAsError.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))); let __go_guard = __go_print_handle.lock().unwrap(); (*__go_guard).clone() }; let __text = match __go_print_value.as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut cycle = Arc::new(Mutex::new(Some(cycleError { next: Default::default() })));
    let mut cycleAsError: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(Some(Arc::new(__go_error_ptr_cycleError(cycle.clone())) as Arc<dyn StdError + Send + Sync>)));
    { let __rhs_holder = cycleAsError.clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *(*cycle.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = new_val; };
    println!("{}", format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = cycleAsError.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = cycleAsError.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())));
    let mut cycleTarget: Arc<Mutex<Option<cycleError>>> = Arc::new(Mutex::new(None));
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some({ let mut __as_found = None; go_error_chain_visit({ let __err_handle = { let __err_source = cycleAsError.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, |__as_err| { if let Some(__as_value) = __as_err.downcast_ref::<__go_error_ptr_cycleError>() { __as_found = Some(__as_value.0.clone()); true } else { false } }); match __as_found { Some(__as_value) => { let new_val = __as_value; cycleTarget = new_val; true } None => false } }))).lock().unwrap().as_ref().unwrap())), format!("{}", {
        let __left = cycleTarget.clone();
        let __right = cycle.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }));
}

pub fn __go_nil_recv_cycle_error_error(__recv: Arc<Mutex<Option<cycleError>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some("cycle".to_string())))
}

pub fn __go_nil_recv_cycle_error_unwrap(err: Arc<Mutex<Option<cycleError>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    { let __err_source = { let __field = (*err.lock().unwrap().as_ref().unwrap()).next.clone(); __field }; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }
}

pub fn __go_nil_recv_nil_safe_error_error(__recv: Arc<Mutex<Option<nilSafeError>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some("custom".to_string())))
}

pub fn __go_nil_recv_nil_safe_error_unwrap(__recv: Arc<Mutex<Option<nilSafeError>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("inner".to_string())))))
}

pub fn __go_nil_recv_pointer_error_error(err: Arc<Mutex<Option<pointerError>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("pointer:{}", (*{ let __field = (*err.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_zero_globals();
    self::__go_init_order_0();
}
