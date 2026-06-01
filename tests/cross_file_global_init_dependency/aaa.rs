use crate::bbb::*;

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

pub static A: GoGlobal<i32> = GoGlobal::new();

pub static C: GoGlobal<i32> = GoGlobal::new();


fn __go_init_globals() {
    *A.borrow_mut() = Some(0);
    *C.borrow_mut() = Some(0);
    *A.borrow_mut() = Some(1);
    *C.borrow_mut() = Some((*B.borrow().as_ref().unwrap()) + 1);
}


pub(crate) fn __go_zero_globals() {
    *A.borrow_mut() = Some(0);
    *C.borrow_mut() = Some(0);
}


pub(crate) fn __go_init_order_0() {
    *A.borrow_mut() = Some(1);
}


pub(crate) fn __go_init_order_2() {
    *C.borrow_mut() = Some((*B.borrow().as_ref().unwrap()) + 1);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all_aaa() {
    self::__go_init_globals();
}
