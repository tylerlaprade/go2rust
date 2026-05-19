use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct os_File;

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}


impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}


pub mod os {
    use super::*;
    pub fn pipe() -> (Arc<Mutex<Option<os_File>>>, Arc<Mutex<Option<os_File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<os_File>(Default::default()))), Arc::new(Mutex::new(Some::<os_File>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


fn main() {
    let (mut read, mut write, mut err) = os::pipe();
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "pipe error".to_string()));
        return;
    }
    let mut err = (*read.lock().unwrap().as_mut().unwrap()).close();
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "read close error".to_string()));
        return;
    }
    let mut err = (*write.lock().unwrap().as_mut().unwrap()).close();
    if (*err.lock().unwrap()).is_some() {
        println!("{}", format!("{}", "write close error".to_string()));
        return;
    }
    println!("{}", format!("{}", "closed".to_string()));
}