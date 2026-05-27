use std::sync::{Arc, Mutex};

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


pub fn make_qualifier() -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>> {
    return Arc::new(Mutex::new(Some(Box::new(move |pkg: Arc<Mutex<Option<types_Package>>>| -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("".to_string())))
    }) as Box<dyn FnMut(Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
}

pub fn use_qualifier(qualifier: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = qualifier.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None))) }
}

pub fn forward_qualifier(qualifier: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    use_qualifier(qualifier.clone())
}

fn main() {
    println!("{}", format!("{}", format!("{}{}", "qualifier:".to_string(), (*forward_qualifier(make_qualifier()).lock().unwrap().as_ref().unwrap()))));
}