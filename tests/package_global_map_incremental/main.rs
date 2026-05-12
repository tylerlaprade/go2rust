use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct symbol {
    pub name: Rc<RefCell<Option<String>>>,
    pub kind: Rc<RefCell<Option<i32>>>,
}

impl symbol {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, kind: { let __guard = self.kind.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.kind.borrow().as_ref().unwrap()))
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

pub(crate) static symbols: GoGlobal<BTreeMap<String, Rc<RefCell<Option<Vec<symbol>>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *symbols.borrow_mut() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<String, Rc<RefCell<Option<Vec<symbol>>>>>::new();
        let __go_map_key_119 = "fmt".to_string();
        let mut __go_map_value_119 = Vec::<symbol>::new();
        __go_map_value_119.push(symbol { name: Rc::new(RefCell::new(Some("Println".to_string()))), kind: Rc::new(RefCell::new(Some(1))), ..Default::default() });
        __go_map_value_119.push(symbol { name: Rc::new(RefCell::new(Some("Printf".to_string()))), kind: Rc::new(RefCell::new(Some(1))), ..Default::default() });
        __go_map.insert(__go_map_key_119, Rc::new(RefCell::new(Some(__go_map_value_119))));
        let __go_map_key_196 = "strings".to_string();
        let mut __go_map_value_196 = Vec::<symbol>::new();
        __go_map_value_196.push(symbol { name: Rc::new(RefCell::new(Some("Builder".to_string()))), kind: Rc::new(RefCell::new(Some(2))), ..Default::default() });
        __go_map_value_196.push(symbol { name: Rc::new(RefCell::new(Some("TrimSpace".to_string()))), kind: Rc::new(RefCell::new(Some(1))), ..Default::default() });
        __go_map.insert(__go_map_key_196, Rc::new(RefCell::new(Some(__go_map_value_196))));
        let __go_map_key_274 = "bytes".to_string();
        let mut __go_map_value_274 = Vec::<symbol>::new();
        __go_map_value_274.push(symbol { name: Rc::new(RefCell::new(Some("Buffer".to_string()))), kind: Rc::new(RefCell::new(Some(2))), ..Default::default() });
        __go_map.insert(__go_map_key_274, Rc::new(RefCell::new(Some(__go_map_value_274))));
        *symbols.borrow_mut() = Some(__go_map);
    }
}


fn main() {
    __go_init_all();
    println!("{} {} {} {}", (*symbols.borrow().as_ref().unwrap()).len(), (*(*symbols.borrow().as_ref().unwrap()).get(&"fmt".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).len(), (*(*(*symbols.borrow().as_ref().unwrap()).get(&"strings".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap())[(0) as usize].clone().name.borrow().as_ref().unwrap()), (*(*(*symbols.borrow().as_ref().unwrap()).get(&"bytes".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap())[(0) as usize].clone().kind.borrow().as_ref().unwrap()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
