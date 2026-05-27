use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Named;

impl std::fmt::Display for types_Named {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Named>")
    }
}


impl types_Named {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Pointer;

impl std::fmt::Display for types_Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Pointer>")
    }
}


impl types_Pointer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct types_Type {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl types_Type {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for types_Type {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
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


impl From<types_Named> for types_Type {
    fn from(_value: types_Named) -> Self {
        Self::__go_from(_value)
    }
}


impl From<types_Pointer> for types_Type {
    fn from(_value: types_Pointer) -> Self {
        Self::__go_from(_value)
    }
}


pub mod types {
    use super::*;
    pub fn new_pointer<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Pointer>>> {
        panic!("new_pointer bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub fn forms(named: Arc<Mutex<Option<types_Named>>>) -> i32 {
    if (*named.lock().unwrap()).is_none() {
        return 0;
    }
    let mut count = Arc::new(Mutex::new(Some(0)));
    for recv in &Vec::<types_Type>::from([{ let __arg = named.clone(); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(types_Type::default) }, { let __arg = types::new_pointer(named.clone()); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(types_Type::default) }]) {
        if true {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }
}

fn main() {
    println!("{}", format!("{}", "ok".to_string()));
}