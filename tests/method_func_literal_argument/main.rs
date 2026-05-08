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
    pub fn run(&self, f: Arc<Mutex<Option<Box<dyn Fn() -> () + Send + Sync>>>>) {
        { let __f_guard = f.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut r = Arc::new(Mutex::new(Some(runner {  })));
    (*r.lock().unwrap().as_mut().unwrap()).run(Arc::new(Mutex::new(Some(Box::new(move || {
        println!("{}", "ran".to_string());
    }) as Box<dyn Fn() -> () + Send + Sync>))));
}