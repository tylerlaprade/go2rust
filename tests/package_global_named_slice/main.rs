use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Items(pub Rc<RefCell<Option<Vec<String>>>>);


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

pub(crate) static All: GoGlobal<Items> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *All.borrow_mut() = Some(Default::default());
    *All.borrow_mut() = Some(Items(Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()])))));
}


pub fn first(xs: Rc<RefCell<Option<Items>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some({ let __seq_holder = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() })));
}

pub fn grow(xs: Rc<RefCell<Option<Items>>>) -> Rc<RefCell<Option<Items>>> {

    return { let __base = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push("gamma".to_string()); Rc::new(RefCell::new(Some(Items(Rc::new(RefCell::new(Some(__values))))))) };
}

fn main() {
    __go_init_all();
    let mut grown = grow(Rc::new(RefCell::new(Some((*All.borrow().as_ref().unwrap()).clone()))));
    println!("{} {} {} {}", { let __slice_holder = { let __named_slice = (*All.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }, (*first(Rc::new(RefCell::new(Some((*All.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()), { let __slice_holder = { let __named_slice = (*grown.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }, { let __seq_holder = { let __named_slice = (*grown.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(2) as usize].clone() });
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
}
