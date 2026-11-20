use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct finder {
    pub base: Arc<Mutex<Option<i64>>>,
}

impl finder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
        }
    }
}


impl Default for finder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            base: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for finder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl finder {
    pub fn find(&self, delta: Arc<Mutex<Option<i64>>>) -> i64 {
        return { let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
    }
}

pub fn call_find() -> i64 {
    {
        let __recv = (Arc::new(Mutex::new(Some(finder { base: Arc::new(Mutex::new(Some(2))), ..Default::default() }))));
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).find(
            Arc::new(Mutex::new(Some(3))),
        );
        __result
    }
}

fn main() {
    if false {
        std::thread::spawn(move || {
        ;
    });
    }
    println!("{}", format!("{}", call_find()));
}