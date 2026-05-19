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

pub(crate) static current: GoGlobal<BTreeMap<String, Rc<RefCell<Option<String>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *current.borrow_mut() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<String>>>>::new();
        __go_map.insert("a".to_string(), Rc::new(RefCell::new(Some("old".to_string()))));
        *current.borrow_mut() = Some(__go_map);
    }
}


fn main() {
    __go_init_all();
    let mut old = Rc::new(RefCell::new(Some((*current.borrow().as_ref().unwrap()).clone())));
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some("new".to_string()))))])))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *current.borrow_mut() = new_val; };
    { let __map_key = "b".to_string(); let __map_value = Rc::new(RefCell::new(Some("saved".to_string()))); (*old.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", (*current.borrow().as_ref().unwrap()).get(&"a".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new())));
    println!("{} {}", format!("{}", (*old.borrow().as_ref().unwrap()).get(&"a".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new())), format!("{}", (*old.borrow().as_ref().unwrap()).get(&"b".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
