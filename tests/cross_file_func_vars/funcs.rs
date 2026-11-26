use crate::{GoFuncValue, GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
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

pub static ProcessData: GoGlobal<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>> = GoGlobal::new();

pub static CombineStrings: GoGlobal<Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>> = GoGlobal::new();

pub static ApplyTwice: GoGlobal<Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>, Rc<RefCell<Option<i64>>>) -> i64>> = GoGlobal::new();

pub static GetGreeting: GoGlobal<Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>> = GoGlobal::new();

pub static DivMod: GoGlobal<Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> (i64, i64)>> = GoGlobal::new();

pub static DynamicFunc: GoGlobal<Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>> = GoGlobal::new();


fn __go_init_globals() {
    self::__go_zero_globals();
    self::__go_init_order_0();
    self::__go_init_order_1();
    self::__go_init_order_2();
    self::__go_init_order_3();
    self::__go_init_order_4();
}


pub(crate) fn __go_zero_globals() {
}


pub(crate) fn __go_init_order_0() {
    *ProcessData.borrow_mut() = Some(Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * 2
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>);
}


pub(crate) fn __go_init_order_1() {
    *CombineStrings.borrow_mut() = Some(Rc::new(move |a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*a.borrow().as_ref().unwrap())));
            __s.push_str(&format!("{}", " ".to_string()));
            __s.push_str(&format!("{}", (*b.borrow().as_ref().unwrap())));
            __s
        })))
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>);
}


pub(crate) fn __go_init_order_2() {
    *ApplyTwice.borrow_mut() = Some(Rc::new(move |f: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>, x: Rc<RefCell<Option<i64>>>| -> i64 {
        { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = f.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = f.borrow().as_ref().unwrap().clone(); (__f)(x.clone()) })))) }
    }) as Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>, Rc<RefCell<Option<i64>>>) -> i64>);
}


pub(crate) fn __go_init_order_3() {
    *GetGreeting.borrow_mut() = Some(Rc::new(move || -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("Hello from function variable!".to_string())))
    }) as Rc<dyn Fn() -> Rc<RefCell<Option<String>>>>);
}


pub(crate) fn __go_init_order_4() {
    *DivMod.borrow_mut() = Some(Rc::new(move |a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>| -> (i64, i64) {
        ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) % (*b.borrow().as_ref().unwrap()))
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> (i64, i64)>);
}


/// Regular function for comparison
pub fn regular_double(x: Rc<RefCell<Option<i64>>>) -> i64 {
    (*x.borrow().as_ref().unwrap()) * 2
}

/// Function that returns a function
pub fn make_multiplier(factor: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>> {
    let factor_closure_clone = factor.clone(); return Rc::new(RefCell::new(Some(Rc::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * (*factor_closure_clone.borrow().as_ref().unwrap())
    }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>)));
}

fn __go_init_0() {
        // Assign function to variable in init
    { let new_val = Rc::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Dynamic: {}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }))))
    }) as Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>; *DynamicFunc.borrow_mut() = Some(new_val); };
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all_funcs() {
    self::__go_init_globals();
    self::__go_init_0();
}
