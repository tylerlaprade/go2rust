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

pub(crate) static currentValues: GoGlobal<Vec<i32>> = GoGlobal::new();

pub(crate) static currentSeen: GoGlobal<BTreeMap<String, Rc<RefCell<Option<i32>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *currentValues.borrow_mut() = Some(vec![]);
    *currentSeen.borrow_mut() = Some(BTreeMap::new());
}


pub fn nil_values() -> Rc<RefCell<Option<Vec<i32>>>> {
    Rc::new(RefCell::new(None))
}

pub fn nil_seen() -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>> {
    Rc::new(RefCell::new(None))
}

pub fn restore(values: Rc<RefCell<Option<Vec<i32>>>>, seen: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>) {
    { let new_val = { let __collection_holder = values.clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *currentValues.borrow_mut() = new_val; };
    { let new_val = { let __collection_holder = seen.clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *currentSeen.borrow_mut() = new_val; };
}

fn main() {
    __go_init_all();
    restore(nil_values(), nil_seen());
    println!("{} {} {} {}", format!("{}", (*currentValues.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*currentValues.borrow()).is_none()), format!("{}", (*currentSeen.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*currentSeen.borrow()).is_none()));

    restore(Rc::new(RefCell::new(Some(vec![1, 2]))), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(3))))])))));
    println!("{} {} {} {}", format!("{}", (*currentValues.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*currentValues.borrow()).is_none()), format!("{}", (*currentSeen.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*currentSeen.borrow()).is_none()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
