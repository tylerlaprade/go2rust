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

pub(crate) static active: GoGlobal<Vec<i64>> = GoGlobal::new();


fn __go_init_globals() {
    *active.borrow_mut() = Some(vec![]);
}


pub fn swap(next: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    let mut old = Rc::new(RefCell::new(Some((*active.borrow().as_ref().unwrap()).clone())));
    { let new_val = { let __collection_holder = next.clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *active.borrow_mut() = new_val; };
    return (((((*old.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) * (10 as i64) as i64) + ((*active.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) as i64) + ((*old.borrow().as_ref().unwrap())[(0) as usize].clone() as i64) as i64) + ((*active.borrow().as_ref().unwrap())[(0) as usize].clone() as i64);
}

fn main() {
    __go_init_all();
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(vec![1i64, 2i64]))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *active.borrow_mut() = new_val; };
    println!("{}", format!("{}", swap(Rc::new(RefCell::new(Some(vec![3i64, 4i64, 5i64]))))));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
