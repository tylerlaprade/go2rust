use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct binary_littleEndian;

impl std::fmt::Display for binary_littleEndian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<binary_littleEndian>")
    }
}


impl binary_littleEndian {
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


pub mod binary {
    use super::*;
    pub fn LittleEndian() -> Arc<Mutex<Option<binary_littleEndian>>> {
        Arc::new(Mutex::new(Some::<binary_littleEndian>(Default::default())))
    }

    pub fn write<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}


pub mod io {
    use super::*;
    pub fn Discard() -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }

    pub fn multi_writer<T0>(_arg0: T0) -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }
}


fn main() {
    let mut out = io::multi_writer((io::Discard().clone(),));
    let out_closure_clone = out.clone(); let mut write = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<u32>>>| {
        let _ = binary::write(out_closure_clone.clone(), binary::LittleEndian().clone(), x.clone());
    }) as Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync>)));
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync> = { let mut __f_guard = write.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(7)))) };
    println!("{}", "ok".to_string());
}