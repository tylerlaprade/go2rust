use std::sync::{Arc, Mutex};

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


impl From<types_Named> for types_Type {
    fn from(_value: types_Named) -> Self {
        Self::default()
    }
}


impl From<types_Pointer> for types_Type {
    fn from(_value: types_Pointer) -> Self {
        Self::default()
    }
}


pub mod types {
    use super::*;
    pub fn new_pointer<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Pointer>>> {
        Arc::new(Mutex::new(Some::<types_Pointer>(Default::default())))
    }
}


pub fn forms(named: Arc<Mutex<Option<types_Named>>>) -> Arc<Mutex<Option<i32>>> {

    if (*named.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(Some(0)));
    }
    let mut count = Arc::new(Mutex::new(Some(0)));
    for recv in &Vec::<types_Type>::from([{ let __arg = named.clone(); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }, { let __arg = types::new_pointer(named.clone()); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }]) {
        if true {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    return count.clone();
}

fn main() {
    println!("{}", "ok".to_string());
}