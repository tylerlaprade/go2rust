use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Default)]
pub struct holder {
    pub value: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


impl holder {
    pub fn to_string(&mut self) -> Arc<Mutex<Option<String>>> {
        return self.value.clone();
    }

    pub fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        return self.to_string();
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut h = Arc::new(Mutex::new(Some(holder { value: Arc::new(Mutex::new(Some("ok".to_string()))), ..Default::default() })));
    println!("{}", (*(*h.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()));
}