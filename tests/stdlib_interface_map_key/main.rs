use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct types_Object {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl types_Object {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn pkg(&self) -> Rc<RefCell<Option<types_Package>>> {
        Rc::new(RefCell::new(Some::<types_Package>(Default::default())))
    }
}

impl Default for types_Object {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for types_Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Object>")
    }
}

impl std::fmt::Display for types_Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Object>")
    }
}

impl PartialEq for types_Object {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Object {}

impl PartialOrd for types_Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_TypeName;

impl std::fmt::Display for types_TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_TypeName>")
    }
}


impl types_TypeName {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<types_TypeName> for types_Object {
    fn from(_value: types_TypeName) -> Self {
        Self::__go_from(_value)
    }
}


#[derive(Debug, Clone, Default)]
pub struct entry {
    pub obj: Rc<RefCell<Option<types_Object>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone() }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.obj.borrow().as_ref().unwrap()))
    }
}


pub fn remember(names: Rc<RefCell<Option<BTreeMap<types_Object, Rc<RefCell<Option<String>>>>>>>, obj: Rc<RefCell<Option<types_TypeName>>>) {
    { let __map_key = { let __arg = obj.clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: types_Object = (*__arg_guard.as_ref().unwrap()).clone().into(); __converted }; __converted }; let __map_value = Rc::new(RefCell::new(Some("name".to_string()))); (*names.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    let _ = (*names.borrow().as_ref().unwrap()).get(&{ let __arg = obj.clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: types_Object = (*__arg_guard.as_ref().unwrap()).clone().into(); __converted }; __converted }).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new());
    let mut entries: Rc<RefCell<Option<Vec<entry>>>> = Rc::new(RefCell::new(None));
    for (__range_key, _) in { let __range_holder = names.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        let key = Rc::new(RefCell::new(Some(__range_key.clone())));
        let _ = (*key.borrow().as_ref().unwrap()).pkg();
        { let new_val = { let __append_target = entries.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(entry { obj: key.clone(), ..Default::default() }); __append_target.clone() }; entries = new_val; };
    }
    let _ = (*entries.borrow().as_ref().unwrap());
    let mut copied = Rc::new(RefCell::new(Some(BTreeMap::<types_Object, Rc<RefCell<Option<String>>>>::from([]))));
    for (__range_key, name) in { let __range_holder = names.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        let key = Rc::new(RefCell::new(Some(__range_key.clone())));
        { let __map_key = (*key.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some((*name.borrow().as_ref().unwrap()).clone()))); (*copied.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        let _ = (*copied.borrow().as_ref().unwrap()).get(&(*key.borrow().as_ref().unwrap()).clone()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new());
    }
}

fn main() {
    if false {
        remember(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)));
    }
    println!("{}", "ok".to_string());
}