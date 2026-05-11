use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct hash_Hash;

impl std::fmt::Display for hash_Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<hash_Hash>")
    }
}


impl hash_Hash {
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


pub mod io {
    use super::*;
    pub fn Discard() -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }

    pub fn multi_writer<T0>(_arg0: T0) -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }
}


pub mod md5 {
    use super::*;
    pub fn new() -> Arc<Mutex<Option<hash_Hash>>> {
        Arc::new(Mutex::new(Some::<hash_Hash>(Default::default())))
    }
}


fn main() {
    io::multi_writer(({ let __selector_holder = io::Discard().clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, md5::new()));
    println!("{}", "ok".to_string());
}