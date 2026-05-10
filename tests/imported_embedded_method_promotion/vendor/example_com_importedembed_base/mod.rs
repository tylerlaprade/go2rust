use go2rust_stdlib_stubs::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct Decoder {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct PkgDecoder {
    pub base: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for PkgDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.base.lock().unwrap().as_ref().unwrap()))
    }
}


impl Decoder {
    pub fn add(&mut self, n: Arc<Mutex<Option<i32>>>) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*n.lock().unwrap().as_ref().unwrap())); };
    }

    pub fn label(&self, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}:{}", { let __v = (*prefix.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*self.value.lock().unwrap().as_ref().unwrap())))));
    }

    pub fn snapshot(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }

    pub fn clone(&self) -> Arc<Mutex<Option<Decoder>>> {
        return Arc::new(Mutex::new(Some(Decoder { value: self.value.clone(), ..Default::default() })));
    }
}

impl PkgDecoder {
    pub fn new_decoder(&self, delta: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Decoder>>> {
        return Arc::new(Mutex::new(Some(Decoder { value: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.base.clone().lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), ..Default::default() })));
    }

    pub fn retire_decoder(&self, d: Arc<Mutex<Option<Decoder>>>) {
        let _ = self;
        let _ = (*d.lock().unwrap().as_ref().unwrap());
    }
}