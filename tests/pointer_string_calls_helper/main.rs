use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct holder {
    pub value: Arc<Mutex<Option<String>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for holder {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl holder {
    pub fn to_string(&self) -> Arc<Mutex<Option<String>>> {
        return self.value.clone();
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        self.to_string()
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut h = Arc::new(Mutex::new(Some(holder { value: Arc::new(Mutex::new(Some("ok".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*{ let __recv = h.clone(); let __recv_ptr: *const holder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const holder }; let __result = unsafe { &*__recv_ptr }.string(); __result }.lock().unwrap().as_ref().unwrap())));
}