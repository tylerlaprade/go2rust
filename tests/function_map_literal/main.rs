use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub type handler = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>;


#[derive(Debug, Clone)]
pub struct item {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


pub type ptrHandler = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>>>>>>;


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

pub(crate) static handlers: GoGlobal<BTreeMap<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>>> = GoGlobal::new();

pub(crate) static ptrHandlers: GoGlobal<BTreeMap<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>>>>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *handlers.borrow_mut() = Some(BTreeMap::new());
    *ptrHandlers.borrow_mut() = Some(BTreeMap::new());
}


pub fn inc(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 1;
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

pub fn twice(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn read(p: Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some({ let __selector_holder = (*p.borrow().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

fn __go_init_0() {
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>>::from([("inc".to_string(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { inc(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))), ("twice".to_string(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { twice(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))))])))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *handlers.borrow_mut() = new_val; };
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>>>>>>>::from([("read".to_string(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<item>>>| -> Rc<RefCell<Option<i32>>> { read(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>>>))))])))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *ptrHandlers.borrow_mut() = new_val; };
}

fn main() {
    __go_init_all();
    let mut a = { let __f_holder = (*handlers.borrow().as_ref().unwrap()).get(&"inc".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) };
    let mut b = { let __f_holder = (*handlers.borrow().as_ref().unwrap()).get(&"twice".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5)))) };
    let mut c = { let __f_holder = (*ptrHandlers.borrow().as_ref().unwrap()).get(&"read".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>>> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(item { value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() })))) };
    eprintln!("{}", format!("{}", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }));
    eprintln!("{}", format!("{}", { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }));
    eprintln!("{}", format!("{}", { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
