use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct runner {
}

impl runner {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
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
    { let __recv = r.clone(); let __recv_ptr: *const runner = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const runner }; let __result = unsafe { &*__recv_ptr }.run(Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", format!("{}", "ran".to_string()));
    }) as Box<dyn FnMut() -> () + Send + Sync>)))); __result };
}