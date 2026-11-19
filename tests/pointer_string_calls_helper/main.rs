use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct holder {
    pub value: Arc<Mutex<Option<String>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for holder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl holder {
    pub fn to_string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
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