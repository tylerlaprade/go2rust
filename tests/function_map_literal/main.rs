use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub type handler = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>;


#[derive(Debug, Clone)]
pub struct item {
    pub value: Rc<RefCell<Option<i64>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub type ptrHandler = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> i64>>>>;


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

pub(crate) static handlers: GoGlobal<BTreeMap<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>>> = GoGlobal::new();

pub(crate) static ptrHandlers: GoGlobal<BTreeMap<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> i64>>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *handlers.borrow_mut() = Some(BTreeMap::new());
    *ptrHandlers.borrow_mut() = Some(BTreeMap::new());
}


pub fn inc(x: Rc<RefCell<Option<i64>>>) -> i64 {
    (*x.borrow().as_ref().unwrap()) + 1
}

pub fn twice(x: Rc<RefCell<Option<i64>>>) -> i64 {
    (*x.borrow().as_ref().unwrap()) * 2
}

pub fn read(p: Rc<RefCell<Option<item>>>) -> i64 {
    return (*(*p.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap());
}

fn __go_init_0() {
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>>::from([("inc".to_string(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { inc(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>)))), ("twice".to_string(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { twice(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))))])))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *handlers.borrow_mut() = new_val; };
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> i64>>>>>::from([("read".to_string(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<item>>>| -> i64 { read(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> i64>))))])))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *ptrHandlers.borrow_mut() = new_val; };
}

fn main() {
    __go_init_all();
    let mut a = { let __f_holder = { let __map_holder = handlers.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"inc".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4)))) };
    let mut b = { let __f_holder = { let __map_holder = handlers.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"twice".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5)))) };
    let mut c = { let __f_holder = { let __map_holder = ptrHandlers.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"read".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> i64> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<item>>>) -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(item { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })))) };
    {
            let __go_print_arg_0 = format!("{}", a);
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", b);
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", c);
            eprintln!("{}", __go_print_arg_0)
        };
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
