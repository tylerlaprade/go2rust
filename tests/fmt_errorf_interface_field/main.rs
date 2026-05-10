use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bytes_Buffer;

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bytes_Buffer>")
    }
}


impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct io_Writer;

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}


impl io_Writer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<bytes_Buffer> for io_Writer {
    fn from(_value: bytes_Buffer) -> Self {
        Self::default()
    }
}


pub mod bytes {
    use super::*;
    pub fn new_buffer<T0>(_arg0: T0) -> Arc<Mutex<Option<bytes_Buffer>>> {
        Arc::new(Mutex::new(Some::<bytes_Buffer>(Default::default())))
    }
}


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub w: Arc<Mutex<Option<io_Writer>>>,
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.w.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut h = Arc::new(Mutex::new(Some(holder { w: { let __arg = bytes::new_buffer(()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() })));
    let mut err = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}", format!("{}", (*(*h.lock().unwrap().as_ref().unwrap()).w.lock().unwrap().as_ref().unwrap())))))));
    println!("{}", (*err.lock().unwrap()).is_some());
}