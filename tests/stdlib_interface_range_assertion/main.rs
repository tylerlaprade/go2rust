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


#[derive(Debug, Clone, Default)]
pub struct exporter {
}

impl exporter {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for exporter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl exporter {
    pub fn accept(&self, obj: Rc<RefCell<Option<types_Object>>>) {
        let _ = (*obj.borrow().as_ref().unwrap());
    }
}

pub fn count_type_names(objs: Rc<RefCell<Option<Vec<types_Object>>>>) -> i32 {
    let mut count = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = objs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for o in __range_values.iter() {
        {
        let (_, mut ok) = ({
        let val = o.clone();
        if let Some(typed_val) = val.downcast_ref::<types_TypeName>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); };
        }
    }
    } }
    return (*count.borrow().as_ref().unwrap());
}

pub fn accept_object_keys(index: Rc<RefCell<Option<BTreeMap<types_Object, Rc<RefCell<Option<u64>>>>>>>, e: Rc<RefCell<Option<exporter>>>) {
    for (__range_key, _) in { let __range_holder = index.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let obj = Rc::new(RefCell::new(Some(__range_key.clone())));
        (*e.borrow().as_ref().unwrap()).accept(obj.clone());
    }
}

fn main() {
    if false {
        println!("{}", format!("{}", count_type_names(Rc::new(RefCell::new(None)))));
        accept_object_keys(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)));
    }
    println!("{}", format!("{}", "ok".to_string()));
}