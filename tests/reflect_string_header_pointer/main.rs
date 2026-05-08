use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct reflect_StringHeader {
    pub data: Arc<Mutex<Option<usize>>>,
    pub len: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for reflect_StringHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_StringHeader>")
    }
}


impl reflect_StringHeader {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut value: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut hdr = Arc::new(Mutex::new(Some(reflect_StringHeader::default())));
    { let new_val = Arc::new(Mutex::new(Some(0 as usize))); *(*hdr.lock().unwrap().as_ref().unwrap()).data.lock().unwrap() = new_val.lock().unwrap().take(); };
    { let new_val = 3; *(*hdr.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
    println!("{}", (*{ let __field = (*hdr.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()));
}