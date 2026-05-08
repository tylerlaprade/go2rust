use std::sync::{Arc, Mutex};

pub fn address(p: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {

    return Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))).lock().unwrap().as_ref().unwrap()) as usize)));
}

fn main() {
    if false {
        let mut x = Arc::new(Mutex::new(Some(1)));
        println!("{}", (*address(x.clone()).lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}