use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl types_Type {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}

impl PartialEq for types_Type {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for types_Type {}

impl PartialOrd for types_Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for types_Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


pub fn values() -> Rc<RefCell<Option<Vec<types_Type>>>> {
    let mut typ: Rc<RefCell<Option<types_Type>>> = Rc::new(RefCell::new(None));
    return Rc::new(RefCell::new(Some(Vec::<types_Type>::from([(*typ.borrow().as_ref().unwrap()).clone()]))));
}

fn main() {
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<u64, Rc<RefCell<Option<types_Type>>>>::new())));
    if false {
        { let __range_holder = values().clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, typ) in __range_values.iter().enumerate() {
        { let __map_key = { let __v = Rc::new(RefCell::new(Some(i as u64))); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }; let __map_value = Rc::new(RefCell::new(Some((*typ).clone()))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    }
    println!("{}", format!("{}", "ok".to_string()));
}