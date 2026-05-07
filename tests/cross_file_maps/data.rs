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
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new();
        __go_map.insert("alice".to_string(), Rc::new(RefCell::new(Some(1))));
        __go_map.insert("bob".to_string(), Rc::new(RefCell::new(Some(2))));
        __go_map.insert("carol".to_string(), Rc::new(RefCell::new(Some(3))));
        *Users.borrow_mut() = Some(__go_map);
    }
    *Numbers.borrow_mut() = Some((*Rc::new(RefCell::new(Some(vec![10, 20, 30, 40, 50]))).borrow().as_ref().unwrap()).clone());
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<Vec<String>>>>>::new();
        let __go_map_key_372 = "admins".to_string();
        let mut __go_map_value_372 = Vec::<String>::new();
        __go_map_value_372.push("alice".to_string());
        __go_map_value_372.push("bob".to_string());
        __go_map.insert(__go_map_key_372, Rc::new(RefCell::new(Some(__go_map_value_372))));
        let __go_map_key_401 = "users".to_string();
        let mut __go_map_value_401 = Vec::<String>::new();
        __go_map_value_401.push("carol".to_string());
        __go_map_value_401.push("dave".to_string());
        __go_map_value_401.push("eve".to_string());
        __go_map.insert(__go_map_key_401, Rc::new(RefCell::new(Some(__go_map_value_401))));
        *Groups.borrow_mut() = Some(__go_map);
    }
    *Records.borrow_mut() = Some((*Rc::new(RefCell::new(Some(vec![BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("name".to_string(), Rc::new(RefCell::new(Some(Box::new("Alice".to_string()) as Box<dyn Any>)))), ("age".to_string(), Rc::new(RefCell::new(Some(Box::new(30) as Box<dyn Any>))))]), BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("name".to_string(), Rc::new(RefCell::new(Some(Box::new("Bob".to_string()) as Box<dyn Any>)))), ("age".to_string(), Rc::new(RefCell::new(Some(Box::new(25) as Box<dyn Any>))))])]))).borrow().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_all() {
    __go_init_globals();
}
