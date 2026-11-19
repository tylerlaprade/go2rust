use std::cell::{RefCell};
use std::collections::BTreeMap;
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

pub(crate) static global: GoGlobal<Vec<i32>> = GoGlobal::new();


fn __go_init_globals() {
    *global.borrow_mut() = Some(vec![]);
}


/// A slice assigned to a package variable, or to an element of a slice of
/// slices, is the same slice afterwards: a write through one name shows
/// through the other. The generated code gives these two targets elements of
/// their own, because a package-level slice and an element of [][]int each
/// hold their elements directly instead of a handle that could be shared
/// (DEV-36). The last case does not compile yet.
fn main() {
    __go_init_all();
    let mut lists = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1, 2]))))]))));
    let mut ok: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));
    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *global.borrow_mut() = __moved_tmp_0; *ok.borrow_mut() = Some(__tmp_1); };
    (*global.borrow_mut().as_mut().unwrap())[(0) as usize] = 7;
    println!("{} {} {} {}", format!("{}", "two results into a package variable".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*global.borrow().as_ref().unwrap())[(0) as usize].clone()));

    let mut dst = Rc::new(RefCell::new(Some(vec![vec![9]])));
    { let (__tmp_0, __tmp_1) = { let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())) { /* MAP_COMMA_OK */ Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; (*dst.borrow_mut().as_mut().unwrap())[(0) as usize] = __tmp_0.borrow_mut().take().unwrap_or_default(); *ok.borrow_mut() = Some(__tmp_1); };
    (*dst.borrow_mut().as_mut().unwrap())[(0) as usize][(1) as usize] = 8;
    println!("{} {} {} {}", format!("{}", "two results into an element".to_string()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __map_holder = lists.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(1) as usize].clone()), format!("{}", (*dst.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone()));

    let mut other = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(vec![1, 2]))))]))));
    { let new_val = { let __collection_holder = { let __map_holder = other.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *global.borrow_mut() = new_val; };
    (*global.borrow_mut().as_mut().unwrap())[(0) as usize] = 7;
    println!("{} {} {}", format!("{}", "one result into a package variable".to_string()), format!("{}", (*{ let __map_holder = other.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*global.borrow().as_ref().unwrap())[(0) as usize].clone()));

    let mut local = Rc::new(RefCell::new(Some(vec![1, 2])));
    { let new_val = { let __collection_holder = local.clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *global.borrow_mut() = new_val; };
    (*global.borrow_mut().as_mut().unwrap())[(0) as usize] = 70;
    println!("{} {} {}", format!("{}", "a local slice into a package variable".to_string()), format!("{}", (*local.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*global.borrow().as_ref().unwrap())[(0) as usize].clone()));

    (*dst.borrow_mut().as_mut().unwrap())[(0) as usize] = (*local.borrow().as_ref().unwrap());
    (*dst.borrow_mut().as_mut().unwrap())[(0) as usize][(1) as usize] = 80;
    println!("{} {} {}", format!("{}", "a local slice into an element".to_string()), format!("{}", (*local.borrow().as_ref().unwrap())[(1) as usize].clone()), format!("{}", (*dst.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
