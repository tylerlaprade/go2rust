use std::any::Any;
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

pub(crate) static Users: GoGlobal<BTreeMap<String, Rc<RefCell<Option<i32>>>>> = GoGlobal::new();

pub(crate) static Numbers: GoGlobal<Vec<i32>> = GoGlobal::new();

pub(crate) static Groups: GoGlobal<BTreeMap<String, Rc<RefCell<Option<Vec<String>>>>>> = GoGlobal::new();

pub(crate) static Records: GoGlobal<Vec<BTreeMap<String, Rc<RefCell<Option<Box<dyn Any>>>>>>> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *Users.borrow_mut() = Some(BTreeMap::new());
    *Numbers.borrow_mut() = Some(vec![]);
    *Groups.borrow_mut() = Some(BTreeMap::new());
    *Records.borrow_mut() = Some(vec![]);
    *Users.borrow_mut() = Some((*Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("alice".to_string(), Rc::new(RefCell::new(Some(1)))), ("bob".to_string(), Rc::new(RefCell::new(Some(2)))), ("carol".to_string(), Rc::new(RefCell::new(Some(3))))])))).borrow().as_ref().unwrap()).clone());
    *Numbers.borrow_mut() = Some((*Rc::new(RefCell::new(Some(vec![10, 20, 30, 40, 50]))).borrow().as_ref().unwrap()).clone());
    *Groups.borrow_mut() = Some((*Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<String>>>>>::from([("admins".to_string(), Rc::new(RefCell::new(Some(vec!["alice".to_string(), "bob".to_string()])))), ("users".to_string(), Rc::new(RefCell::new(Some(vec!["carol".to_string(), "dave".to_string(), "eve".to_string()]))))])))).borrow().as_ref().unwrap()).clone());
    *Records.borrow_mut() = Some((*Rc::new(RefCell::new(Some(vec![BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("name".to_string(), Rc::new(RefCell::new(Some(Box::new("Alice".to_string()) as Box<dyn Any>)))), ("age".to_string(), Rc::new(RefCell::new(Some(Box::new(30) as Box<dyn Any>))))]), BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("name".to_string(), Rc::new(RefCell::new(Some(Box::new("Bob".to_string()) as Box<dyn Any>)))), ("age".to_string(), Rc::new(RefCell::new(Some(Box::new(25) as Box<dyn Any>))))])]))).borrow().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_all() {
    __go_init_globals();
}
