use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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


#[derive(Debug, Clone, Default)]
pub struct cache {
}

impl cache {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl cache {
    pub fn r#use(&self, T: Rc<RefCell<Option<types_Type>>>) {
        let _ = T;
    }
}

pub fn exercise(T: Rc<RefCell<Option<types_Type>>>, c: Rc<RefCell<Option<cache>>>) {
    (*c.borrow_mut().as_mut().unwrap()).r#use(T.clone());
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<types_Type, Rc<RefCell<Option<i32>>>>::from([((*T.borrow().as_ref().unwrap()).clone(), Rc::new(RefCell::new(Some(1))))]))));
    println!("{}", format!("{}", (*seen.borrow().as_ref().unwrap()).get(&(*T.borrow().as_ref().unwrap()).clone()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));
}

fn main() {
    if false {
        exercise(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some(cache {  }))));
    }
    println!("{}", format!("{}", "ok".to_string()));
}