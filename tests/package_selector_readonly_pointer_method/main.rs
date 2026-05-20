use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

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

pub(crate) static local: GoGlobal<Rc<RefCell<Option<example_com_package_selector_readonly_pointer_method_helper::Counter>>>> = GoGlobal::new();


fn __go_init_globals() {
    *local.borrow_mut() = Some(Default::default());
    *local.borrow_mut() = Some(example_com_package_selector_readonly_pointer_method_helper::new_counter());
}


fn main() {
    example_com_package_selector_readonly_pointer_method_helper::__go_init_all();

    __go_init_all();
    println!("{}", format!("{}", (*{ let __recv_holder = (*local.borrow().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.borrow().as_ref().unwrap()).clone(); let __result = __recv_value.total(); __result }.borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
