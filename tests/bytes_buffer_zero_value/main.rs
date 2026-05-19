use std::sync::{Arc, Mutex};
use std::thread;

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
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some::<String>(Default::default())))
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    println!("{}", format!("{}", { let __tmp_x = (*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ""; __tmp_x == __tmp_y }));
}