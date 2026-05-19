use std::sync::{Arc, Mutex};

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
    let mut value = Arc::new(Mutex::new(Some("abc".to_string())));
    let mut hdr = Arc::new(Mutex::new(Some(reflect_StringHeader { data: Arc::new(Mutex::new(Some(0 as usize))), len: Arc::new(Mutex::new(Some({ let __s = (*value.lock().unwrap().as_ref().unwrap()).clone(); __s.len() as i32 }))), ..Default::default() })));
    println!("{}", format!("{}", (*{ let __field = (*hdr.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap())));
}