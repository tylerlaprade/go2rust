#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod detail;
use detail::*;

use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug, Clone, PartialEq)]
pub struct valueError {
    pub code: Rc<RefCell<Option<String>>>,
}

impl valueError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for valueError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for valueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


/// This used to collide with the generated pointerErrorPtr support type.
#[derive(Debug, Clone)]
pub struct pointerErrorPtr {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl pointerErrorPtr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for pointerErrorPtr {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerErrorPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// A reserved-looking Go identifier is still a source name and must also not
/// collide with the generated pointer-error support type.
#[derive(Debug, Clone, Default)]
pub struct __go_error_ptr_pointerError {
}

impl __go_error_ptr_pointerError {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for __go_error_ptr_pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct nestedError {
    pub key: Rc<RefCell<Option<nestedCode>>>,
}

impl nestedError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            key: __go_clone_0_0,
        }
    }
}


impl Default for nestedError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(nestedCode::default())));
        Self {
            key: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for nestedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct nanError {
    pub value: Rc<RefCell<Option<f64>>>,
}

impl nanError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for nanError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for nanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct wrapError {
    pub cause: Rc<RefCell<Option<Rc<dyn StdError>>>>,
}

impl wrapError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.cause.clone();
        Self {
            cause: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for wrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for wrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


#[derive(Debug, Clone, Default)]
pub struct pointerFieldError {
    pub p: Rc<RefCell<Option<i64>>>,
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
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}
impl PartialEq for pointerFieldError {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left_some = self.p.borrow().is_some(); let __right_some = other.p.borrow().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Rc::ptr_eq(&self.p, &other.p)) }
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
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct cycleError {
    pub next: Rc<RefCell<Option<Rc<dyn StdError>>>>,
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
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for cycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static evaluationOrder: GoGlobal<String> = GoGlobal::new();

pub static ErrBusy: GoGlobal<Rc<RefCell<Option<pointerError>>>> = GoGlobal::new();


fn __go_init_globals() {
    *evaluationOrder.borrow_mut() = Some(String::new());
    *ErrBusy.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *ErrBusy.borrow_mut() = Some(Rc::new(RefCell::new(Some(pointerError { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }))));
}


pub(crate) fn __go_zero_globals() {
    *evaluationOrder.borrow_mut() = Some(String::new());
    *ErrBusy.borrow_mut() = Some(Rc::new(RefCell::new(None)));
}


pub(crate) fn __go_init_order_0() {
    *ErrBusy.borrow_mut() = Some(Rc::new(RefCell::new(Some(pointerError { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }))));
}


impl valueError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.code.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for valueError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_valueError(pub Rc<RefCell<Option<valueError>>>);

impl std::fmt::Display for __go_error_ptr_valueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_valueError {}


impl pointerError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("pointer:{}", (*self.n.borrow().as_ref().unwrap())))))
    }
}

impl StdError for pointerError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_pointerError_1(pub Rc<RefCell<Option<pointerError>>>);

impl std::fmt::Display for __go_error_ptr_pointerError_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_previous_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let __go_error_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| __go_nil_recv_pointer_error_error(self.0.clone())));
            std::panic::set_hook(__go_previous_panic_hook);
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_pointerError_1 {}


impl nestedError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = (*self.key.borrow().as_ref().unwrap()).code.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for nestedError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_nestedError(pub Rc<RefCell<Option<nestedError>>>);

impl std::fmt::Display for __go_error_ptr_nestedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_nestedError {}


impl nanError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("nan".to_string())))
    }
}

impl StdError for nanError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_nanError(pub Rc<RefCell<Option<nanError>>>);

impl std::fmt::Display for __go_error_ptr_nanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_nanError {}


impl pointerFieldError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("pointer field".to_string())))
    }
}

impl StdError for pointerFieldError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_pointerFieldError(pub Rc<RefCell<Option<pointerFieldError>>>);

impl std::fmt::Display for __go_error_ptr_pointerFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") }
    }
}

impl StdError for __go_error_ptr_pointerFieldError {}


impl nilSafeError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("custom".to_string())))
    }

    pub fn unwrap(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("inner".to_string())))))
    }
}

impl StdError for nilSafeError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_nilSafeError(pub Rc<RefCell<Option<nilSafeError>>>);

impl std::fmt::Display for __go_error_ptr_nilSafeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_previous_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let __go_error_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| __go_nil_recv_nil_safe_error_error(self.0.clone())));
            std::panic::set_hook(__go_previous_panic_hook);
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_nilSafeError {}


impl cycleError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("cycle".to_string())))
    }

    pub fn unwrap(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        { let __err_source = { let __field = self.next.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
    }
}

impl StdError for cycleError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_cycleError(pub Rc<RefCell<Option<cycleError>>>);

impl std::fmt::Display for __go_error_ptr_cycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_previous_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let __go_error_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| __go_nil_recv_cycle_error_error(self.0.clone())));
            std::panic::set_hook(__go_previous_panic_hook);
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_cycleError {}


impl wrapError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "wrapped: ".to_string(), (*Rc::new(RefCell::new(Some(format!("{}", self.cause.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())))))
    }

    pub fn unwrap(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        { let __err_source = { let __field = self.cause.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
    }
}

impl StdError for wrapError {}

#[derive(Debug, Clone)]
pub struct __go_error_ptr_wrapError(pub Rc<RefCell<Option<wrapError>>>);

impl std::fmt::Display for __go_error_ptr_wrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_is_nil = self.0.borrow().is_none();
        if !__go_is_nil {
            let __guard = self.0.borrow();
            std::fmt::Display::fmt(__guard.as_ref().unwrap(), f)
        } else {
            let __go_previous_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let __go_error_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| __go_nil_recv_wrap_error_error(self.0.clone())));
            std::panic::set_hook(__go_previous_panic_hook);
            match __go_error_result { Ok(__text) => { let __guard = __text.borrow(); match __guard.as_ref() { Some(__v) => std::fmt::Display::fmt(__v, f), None => write!(f, "<nil>") } }, Err(_) => write!(f, "<nil>") }
        }
    }
}

impl StdError for __go_error_ptr_wrapError {}


pub fn pointer_operand(err: Rc<RefCell<Option<pointerError>>>) -> Rc<RefCell<Option<pointerError>>> {
    { (*evaluationOrder.borrow_mut().as_mut().unwrap()).push_str(&"P".to_string()); };
    err.clone()
}

pub fn error_operand(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    { (*evaluationOrder.borrow_mut().as_mut().unwrap()).push_str(&"E".to_string()); };
    { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
}

pub fn return_busy() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1({ let __arg_holder = ErrBusy.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })) as Rc<dyn StdError>)))
}

pub fn same_by_switch(err: Rc<RefCell<Option<Rc<dyn StdError>>>>, target: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> bool {
    { let _switch_val = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    if { let __left_holder = _switch_val.clone(); let __left_value = { (*__left_holder.borrow()).clone() }; let __right_holder = { let __err_source = target.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __right_value = { (*__right_holder.borrow()).clone() }; match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__left), Some(__right)) => if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_cycleError>(), __right.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<nanError>(), __right.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_nanError>(), __right.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<nestedError>(), __right.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_nestedError>(), __right.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_nilSafeError>(), __right.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_pointerError_1>(), __right.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<pointerFieldError>(), __right.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_pointerFieldError>(), __right.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<valueError>(), __right.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_valueError>(), __right.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__left.downcast_ref::<__go_error_ptr_wrapError>(), __right.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__left, &**__right) { true } else { false }, (None, None) => true, _ => false } } {
            return true;
        }
    }
    false
}

fn main() {
    __go_init_all();
    let mut left: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(valueError { code: Rc::new(RefCell::new(Some("same".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    let mut equal: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(valueError { code: Rc::new(RefCell::new(Some("same".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    let mut different: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(valueError { code: Rc::new(RefCell::new(Some("different".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    println!("{} {}", format!("{}", { let __left = left.clone(); let __right = equal.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = left.clone(); let __right = different.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut pointer = Rc::new(RefCell::new(Some(pointerError { n: Rc::new(RefCell::new(Some(1))), ..Default::default() })));
    let mut first: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1(pointer.clone())) as Rc<dyn StdError>)));
    let mut second: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1(pointer.clone())) as Rc<dyn StdError>)));
    println!("{}", format!("{}", { let __left = first.clone(); let __right = second.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));
    { let new_val = 2; *(*pointer.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", first.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", second.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    println!("{}", format!("{}", { let __left = return_busy().clone(); let __right = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1({ let __arg_holder = ErrBusy.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })) as Rc<dyn StdError>))); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));
    println!("{} {}", format!("{}", same_by_switch({ let __err_source = left.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, { let __err_source = equal.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) })), format!("{}", same_by_switch({ let __err_source = first.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, { let __err_source = second.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) })));

    let mut literal: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1(Rc::new(RefCell::new(Some(pointerError { n: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone())) as Rc<dyn StdError>)));
    let (mut literalPointer, mut literalOK) = ({
        let val = literal.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<__go_error_ptr_pointerError_1>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<pointerError>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<pointerError>)), false)
        }
    });
    println!("{} {}", format!("{}", literalOK), format!("{}", (*(*literalPointer.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
    let mut asserted = ({
        let val = first.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<__go_error_ptr_pointerError_1>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
    println!("{}", format!("{}", { let __left = asserted.clone(); let __right = pointer.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq }));

    let mut nilPointer1: Rc<RefCell<Option<pointerError>>> = Rc::new(RefCell::new(None));
    let mut nilPointer2: Rc<RefCell<Option<pointerError>>> = Rc::new(RefCell::new(None));
    let mut nilError1: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1(nilPointer1.clone())) as Rc<dyn StdError>)));
    let mut nilError2: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1(nilPointer2.clone())) as Rc<dyn StdError>)));
    println!("{} {}", format!("{}", { let __left = nilError1.clone(); let __right = nilError2.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __nil_result = (*nilError1.borrow()).is_none(); __nil_result }));

    let mut nestedLeft: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(nestedError { key: Rc::new(RefCell::new(Some(nestedCode { code: Rc::new(RefCell::new(Some("nested".to_string()))), ..Default::default() }))), ..Default::default() }) as Rc<dyn StdError>)));
    let mut nestedRight: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(nestedError { key: Rc::new(RefCell::new(Some(nestedCode { code: Rc::new(RefCell::new(Some("nested".to_string()))), ..Default::default() }))), ..Default::default() }) as Rc<dyn StdError>)));
    println!("{}", format!("{}", { let __left = nestedLeft.clone(); let __right = nestedRight.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut zero = Rc::new(RefCell::new(Some(0.0)));
    let mut nan: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(nanError { value: Rc::new(RefCell::new(Some({ let __bin_zero = (*zero.borrow().as_ref().unwrap()).clone(); __bin_zero / __bin_zero }))), ..Default::default() }) as Rc<dyn StdError>)));
    let mut nanCopy = { let __err_source = nan.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    println!("{} {}", format!("{}", { let __left = nan.clone(); let __right = nan.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = nan.clone(); let __right = nanCopy.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let (mut firstNumber, mut secondNumber) = (Rc::new(RefCell::new(Some(1i64))), Rc::new(RefCell::new(Some(1i64))));
    let mut pointerFieldLeft: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(pointerFieldError { p: firstNumber.clone().clone(), ..Default::default() }) as Rc<dyn StdError>)));
    let mut pointerFieldRight: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(pointerFieldError { p: secondNumber.clone().clone(), ..Default::default() }) as Rc<dyn StdError>)));
    let mut pointerFieldCopy = { let __err_source = pointerFieldLeft.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    println!("{} {}", format!("{}", { let __left = pointerFieldLeft.clone(); let __right = pointerFieldRight.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __left = pointerFieldLeft.clone(); let __right = pointerFieldCopy.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    let mut nilSafe: Rc<RefCell<Option<nilSafeError>>> = Rc::new(RefCell::new(None));
    let mut nilSafeAsError: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_nilSafeError(nilSafe.clone())) as Rc<dyn StdError>)));
    println!("{} {}", format!("{}", { let __text = match nilSafeAsError.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = nilSafeAsError.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut cycle = Rc::new(RefCell::new(Some(cycleError { next: Default::default() })));
    let mut cycleAsError: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_cycleError(cycle.clone())) as Rc<dyn StdError>)));
    { let __rhs_holder = cycleAsError.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *(*cycle.borrow().as_ref().unwrap()).next.borrow_mut() = new_val; };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = cycleAsError.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = cycleAsError.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())));
    let mut cycleTarget: Rc<RefCell<Option<cycleError>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some({ let mut __as_found = None; go_error_chain_visit({ let __err_handle = { let __err_source = cycleAsError.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, |__as_err| { if let Some(__as_value) = __as_err.downcast_ref::<__go_error_ptr_cycleError>() { __as_found = Some(__as_value.0.clone()); true } else { false } }); match __as_found { Some(__as_value) => { let new_val = __as_value; cycleTarget = new_val; true } None => false } }))).borrow().as_ref().unwrap())), format!("{}", { let __left = cycleTarget.clone(); let __right = cycle.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq }));

    let mut leaf = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("leaf".to_string())))));
    let mut wrapped: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_wrapError(Rc::new(RefCell::new(Some(wrapError { cause: { let __err_source = leaf.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, ..Default::default() }))).clone())) as Rc<dyn StdError>)));
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(go_errors_is({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }, { let __err_handle = { let __err_source = leaf.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value })))).borrow().as_ref().unwrap())), format!("{}", { let __left = Rc::new(RefCell::new(go_errors_unwrap({ let __err_handle = { let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }))).clone(); let __right = leaf.clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }));

    { let new_val = "".to_string(); *evaluationOrder.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", { let __right = Rc::new(RefCell::new(Some(Rc::new(__go_error_ptr_pointerError_1(pointer_operand(pointer.clone()).clone())) as Rc<dyn StdError>))); let __left = error_operand({ let __err_source = first.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }).clone(); let __left_value = { (*__left.borrow()).clone() }; let __right_value = { (*__right.borrow()).clone() }; let __eq = match (__left_value.as_ref(), __right_value.as_ref()) { (Some(__l), Some(__r)) => if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_cycleError>(), __r.downcast_ref::<__go_error_ptr_cycleError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nanError>(), __r.downcast_ref::<nanError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nanError>(), __r.downcast_ref::<__go_error_ptr_nanError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<nestedError>(), __r.downcast_ref::<nestedError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nestedError>(), __r.downcast_ref::<__go_error_ptr_nestedError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_nilSafeError>(), __r.downcast_ref::<__go_error_ptr_nilSafeError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerError_1>(), __r.downcast_ref::<__go_error_ptr_pointerError_1>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<pointerFieldError>(), __r.downcast_ref::<pointerFieldError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_pointerFieldError>(), __r.downcast_ref::<__go_error_ptr_pointerFieldError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<valueError>(), __r.downcast_ref::<valueError>()) { __go_left == __go_right } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_valueError>(), __r.downcast_ref::<__go_error_ptr_valueError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if let (Some(__go_left), Some(__go_right)) = (__l.downcast_ref::<__go_error_ptr_wrapError>(), __r.downcast_ref::<__go_error_ptr_wrapError>()) { let __go_left_nil = { __go_left.0.borrow().is_none() }; let __go_right_nil = { __go_right.0.borrow().is_none() }; if __go_left_nil || __go_right_nil { __go_left_nil && __go_right_nil } else { Rc::ptr_eq(&__go_left.0, &__go_right.0) } } else if std::ptr::addr_eq(&**__l, &**__r) { true } else { false }, (None, None) => true, _ => false }; __eq }), format!("{}", { let __v = (*evaluationOrder.borrow().as_ref().unwrap()).clone(); __v }));
}

pub fn __go_nil_recv_cycle_error_error(__recv: Rc<RefCell<Option<cycleError>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("cycle".to_string())))
}

pub fn __go_nil_recv_cycle_error_unwrap(err: Rc<RefCell<Option<cycleError>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    { let __err_source = { let __field = (*err.borrow().as_ref().unwrap()).next.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
}

pub fn __go_nil_recv_nil_safe_error_error(__recv: Rc<RefCell<Option<nilSafeError>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some("custom".to_string())))
}

pub fn __go_nil_recv_nil_safe_error_unwrap(__recv: Rc<RefCell<Option<nilSafeError>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("inner".to_string())))))
}

pub fn __go_nil_recv_pointer_error_error(err: Rc<RefCell<Option<pointerError>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("pointer:{}", (*(*err.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())))))
}

pub fn __go_nil_recv_wrap_error_error(err: Rc<RefCell<Option<wrapError>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}{}", "wrapped: ".to_string(), (*Rc::new(RefCell::new(Some(format!("{}", (*err.borrow().as_ref().unwrap()).cause.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())))))
}

pub fn __go_nil_recv_wrap_error_unwrap(err: Rc<RefCell<Option<wrapError>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    { let __err_source = { let __field = (*err.borrow().as_ref().unwrap()).cause.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_zero_globals();
    self::__go_init_order_0();
}
