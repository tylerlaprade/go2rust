use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Tuple;

impl std::fmt::Display for types_Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Tuple>")
    }
}


impl types_Tuple {
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


impl From<types_Tuple> for types_Type {
    fn from(_value: types_Tuple) -> Self {
        Self::default()
    }
}


pub mod types {
    use super::*;
    pub fn new_tuple<T0>(_arg0: T0) -> Arc<Mutex<Option<types_Tuple>>> {
        Arc::new(Mutex::new(Some::<types_Tuple>(Default::default())))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Walker {
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Walker {
    pub fn has(&self, t: Arc<Mutex<Option<types_Type>>>) -> Arc<Mutex<Option<bool>>> {
        return Arc::new(Mutex::new(Some(true)));
    }
}

pub fn has(t: Arc<Mutex<Option<types_Type>>>) -> Arc<Mutex<Option<bool>>> {

    return Arc::new(Mutex::new(Some(true)));
}

fn main() {
    let mut w: Arc<Mutex<Option<Walker>>> = Arc::new(Mutex::new(Some(Default::default())));

    println!("{}", (*has({ let __arg = types::new_tuple(()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }).lock().unwrap().as_ref().unwrap()));
    println!("{}", (*(*w.lock().unwrap().as_ref().unwrap()).has({ let __arg = types::new_tuple(()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }).lock().unwrap().as_ref().unwrap()));
}