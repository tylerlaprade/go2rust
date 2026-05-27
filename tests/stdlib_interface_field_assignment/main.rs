use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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


pub mod types {
    use super::*;
    pub fn identical<T0, T1>(_arg0: T0, _arg1: T1) -> bool {
        panic!("identical bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone)]
pub struct term {
    pub tilde: Arc<Mutex<Option<bool>>>,
    pub typ: Arc<Mutex<Option<types_Type>>>,
}

impl term {
    pub fn __go_value_clone(&self) -> Self {
        Self { tilde: { let __guard = self.tilde.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone() }
    }
}


impl Default for term {
    fn default() -> Self {
        Self { tilde: Arc::new(Mutex::new(Some(false))), typ: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tilde.lock().unwrap().as_ref().unwrap()), (*self.typ.lock().unwrap().as_ref().unwrap()))
    }
}


pub fn under(t: Arc<Mutex<Option<types_Type>>>) -> Arc<Mutex<Option<types_Type>>> {
    t.clone()
}

pub fn disjoint(x: Arc<Mutex<Option<term>>>, y: Arc<Mutex<Option<term>>>) -> bool {
    let mut ux = { let __src = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __copied = (*__src.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__copied))) };
    if (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = under(ux.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ux.lock().unwrap() = __moved_val; };
    }
    let mut uy = { let __src = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __copied = (*__src.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__copied))) };
    if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = under(uy.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uy.lock().unwrap() = __moved_val; };
    }
    return !types::identical(ux.clone(), uy.clone());
}

fn main() {
    if false {
        let mut t = Arc::new(Mutex::new(Some(term { tilde: Arc::new(Mutex::new(Some(false))), typ: Arc::new(Mutex::new(Some(Default::default()))) })));
        println!("{}", format!("{}", disjoint(t.clone(), t.clone())));
    }
    println!("{}", format!("{}", "ok".to_string()));
}