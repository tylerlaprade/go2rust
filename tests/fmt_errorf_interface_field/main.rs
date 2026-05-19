use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



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


#[derive(Clone)]
pub struct io_Writer {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_Writer {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for io_Writer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl PartialEq for io_Writer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Writer {}

impl PartialOrd for io_Writer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Writer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


impl From<bytes_Buffer> for io_Writer {
    fn from(_value: bytes_Buffer) -> Self {
        Self::__go_from(_value)
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

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { w: self.w.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.w.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut h = Arc::new(Mutex::new(Some(holder { w: { let __arg = bytes::new_buffer(()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() })));
    let mut err = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}", format!("{}", (*(*h.lock().unwrap().as_ref().unwrap()).w.lock().unwrap().as_ref().unwrap())))))));
    println!("{}", format!("{}", (*err.lock().unwrap()).is_some()));
}