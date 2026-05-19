use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct finder {
    pub base: Arc<Mutex<Option<i32>>>,
}

impl finder {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for finder {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for finder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.base.lock().unwrap().as_ref().unwrap()))
    }
}


impl finder {
    pub fn find(&self, delta: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap());
            let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v };
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }
}

pub fn call_find() -> Arc<Mutex<Option<i32>>> {

    return { let __recv = (Arc::new(Mutex::new(Some(finder { base: Arc::new(Mutex::new(Some(2))), ..Default::default() })))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).find(Arc::new(Mutex::new(Some(3)))); __result };
}

fn main() {
    if false {
        std::thread::spawn(move || {
        ;
    });
    }
    println!("{}", (*call_find().lock().unwrap().as_ref().unwrap()));
}