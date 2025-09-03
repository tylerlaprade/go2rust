use std::sync::{Arc, Mutex};

fn main() {
    (*rand.lock().unwrap().as_mut().unwrap())::seed(Arc::new(Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap())::now().unix_nano()))));
    println!("{} {}", "Random int:".to_string(), (*(*rand.lock().unwrap().as_mut().unwrap())::intn(Arc::new(Mutex::new(Some(100)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Random float:".to_string(), (*(*rand.lock().unwrap().as_mut().unwrap())::float64().lock().unwrap().as_ref().unwrap()));
}