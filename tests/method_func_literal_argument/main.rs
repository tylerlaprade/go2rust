use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct runner {
}

impl std::fmt::Display for runner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl runner {
    pub fn run(&self, f: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut r = Arc::new(Mutex::new(Some(runner {  })));
    (*r.lock().unwrap().as_mut().unwrap()).run(Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", "ran".to_string());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}