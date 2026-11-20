use std::sync::{Arc, Mutex};

pub fn address(p: Arc<Mutex<Option<i64>>>) -> usize {
    (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap())
}

fn main() {
    if false {
        let mut x = Arc::new(Mutex::new(Some(1i64)));
        println!("{}", format!("{}", address(x.clone())));
    }
    println!("{}", format!("{}", "ok".to_string()));
}