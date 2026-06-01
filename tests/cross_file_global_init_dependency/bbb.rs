use crate::aaa::*;

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

pub static B: GoGlobal<i32> = GoGlobal::new();


fn __go_init_globals() {
    *B.borrow_mut() = Some(0);
    *B.borrow_mut() = Some((*A.borrow().as_ref().unwrap()) + 1);
}


pub(crate) fn __go_zero_globals() {
    *B.borrow_mut() = Some(0);
}


pub(crate) fn __go_init_order_1() {
    *B.borrow_mut() = Some((*A.borrow().as_ref().unwrap()) + 1);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all_bbb() {
    self::__go_init_globals();
}
