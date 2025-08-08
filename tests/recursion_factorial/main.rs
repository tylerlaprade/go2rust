use std::sync::{Arc, Mutex};

pub fn fact(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) == 0 {
        return Arc::new(Mutex::new(Some(1)));
    }
    return {
            let __tmp_x = (*n.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*fact(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))).lock().unwrap().as_ref().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
}

fn main() {
    println!("{}", (*fact(Arc::new(Mutex::new(Some(7)))).lock().unwrap().as_ref().unwrap()));
}