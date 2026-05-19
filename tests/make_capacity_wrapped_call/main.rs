use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct counter {
    pub n: Arc<Mutex<Option<i32>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for counter {
    fn default() -> Self {
        Self { n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}


impl counter {
    pub fn len(&self) -> Arc<Mutex<Option<i32>>> {
        return self.n.clone();
    }
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut c = Arc::new(Mutex::new(Some(counter { n: Arc::new(Mutex::new(Some(3))), ..Default::default() })));
    let mut xs = Arc::new(Mutex::new(Some(Vec::with_capacity((*(*c.lock().unwrap().as_mut().unwrap()).len().lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = { let __append_target = xs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend(vec![1, 2, 3]); __append_target.clone() }; xs = new_val; };

    println!("{}", format!("{}", (*xs.lock().unwrap().as_ref().unwrap()).len()));
    println!("{}", format!("{}", (*xs.lock().unwrap().as_ref().unwrap()).capacity()));
}