use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Term;

impl std::fmt::Display for types_Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Term>")
    }
}


impl types_Term {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn r#type(&self) -> Arc<Mutex<Option<types_Type>>> {
        Arc::new(Mutex::new(Some::<types_Type>(Default::default())))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Type;

impl std::fmt::Display for types_Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Type>")
    }
}


impl types_Type {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod types {
    use super::*;
    pub fn new_term<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<types_Term>>> {
        Arc::new(Mutex::new(Some::<types_Term>(Default::default())))
    }
}


fn main() {
    let mut terms = Arc::new(Mutex::new(Some(vec![types::new_term(false, ())])));
    { let __recv = { let __seq = { let __seq_holder = terms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).r#type(); __result };
    { let __range_holder = terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for term in __range_values.iter() {
        (*term.lock().unwrap().as_mut().unwrap()).r#type();
    } }
    println!("{}", "ok".to_string());
}