use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Method {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Method {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
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

pub(crate) static fallback: GoGlobal<Vec<Rc<RefCell<Option<Method>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *fallback.borrow_mut() = Some(vec![]);
}


pub fn lookup(methods: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<Rc<RefCell<Option<Method>>>>>>>>>>>, receiver: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<Rc<RefCell<Option<Method>>>>>>> {

    if (*receiver.borrow().as_ref().unwrap()).clone() != "" && ((*(*methods.borrow().as_ref().unwrap()).get(&(*receiver.borrow().as_ref().unwrap()).clone()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).len() as i32) > (0 as i32) {
        return (*methods.borrow().as_ref().unwrap()).get(&(*receiver.borrow().as_ref().unwrap()).clone()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default());
    }
    return fallback.clone();
}

fn main() {
    __go_init_all();
    let mut first = Rc::new(RefCell::new(Some(Method { name: Rc::new(RefCell::new(Some("first".to_string()))), ..Default::default() })));
    let mut second = Rc::new(RefCell::new(Some(Method { name: Rc::new(RefCell::new(Some("second".to_string()))), ..Default::default() })));
    let mut methods = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<Rc<RefCell<Option<Method>>>>>>>>::from([("Thing".to_string(), Rc::new(RefCell::new(Some(vec![first.clone(), second.clone()]))))]))));
    { let new_val = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Method { name: Rc::new(RefCell::new(Some("fallback".to_string()))), ..Default::default() })))]))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *fallback.borrow_mut() = __moved_val; };

    let mut got = lookup(methods.clone(), Rc::new(RefCell::new(Some("Thing".to_string()))));
    let mut emptyReceiver = lookup(methods.clone(), Rc::new(RefCell::new(Some("".to_string()))));
    println!("{} {}", (*got.borrow().as_ref().unwrap()).len(), (*emptyReceiver.borrow().as_ref().unwrap()).len());
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
