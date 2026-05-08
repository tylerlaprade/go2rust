use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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
    pub fn identical<T0, T1>(_arg0: T0, _arg1: T1) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some::<bool>(Default::default())))
    }
}


#[derive(Debug, Clone, Default)]
pub struct term {
    pub tilde: Arc<Mutex<Option<bool>>>,
    pub typ: Arc<Mutex<Option<types_Type>>>,
}

impl std::fmt::Display for term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tilde.lock().unwrap().as_ref().unwrap()), (*self.typ.lock().unwrap().as_ref().unwrap()))
    }
}


pub fn under(t: Arc<Mutex<Option<types_Type>>>) -> Arc<Mutex<Option<types_Type>>> {

    return t.clone();
}

pub fn disjoint(x: Arc<Mutex<Option<term>>>, y: Arc<Mutex<Option<term>>>) -> Arc<Mutex<Option<bool>>> {

    let mut ux = { let __src = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __copied = (*__src.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__copied))) };
    if (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = under(Arc::new(Mutex::new(Some((*ux.lock().unwrap().as_ref().unwrap()).clone())))); *ux.lock().unwrap() = new_val.lock().unwrap().take(); };
    }
    let mut uy = { let __src = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __copied = (*__src.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__copied))) };
    if (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = under(Arc::new(Mutex::new(Some((*uy.lock().unwrap().as_ref().unwrap()).clone())))); *uy.lock().unwrap() = new_val.lock().unwrap().take(); };
    }
    return Arc::new(Mutex::new(Some(!((*types::identical(ux.clone(), uy.clone()).lock().unwrap().as_ref().unwrap())))));
}

fn main() {
    if false {
        let mut t = Arc::new(Mutex::new(Some(term { tilde: Arc::new(Mutex::new(Some(false))), typ: Arc::new(Mutex::new(Some(Default::default()))) })));
        println!("{}", (*disjoint(t.clone(), t.clone()).lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}