use std::sync::{Arc, Mutex};

fn main() {
    (*rand.lock().unwrap().as_ref().unwrap())::seed(Arc::new(Mutex::new(Some((*time.lock().unwrap().as_ref().unwrap())::now().unix_nano()))));
    println!("{} {}", "Random int:".to_string(), (*(*rand.lock().unwrap().as_ref().unwrap())::intn(Arc::new(Mutex::new(Some(100)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Random float:".to_string(), (*(*rand.lock().unwrap().as_ref().unwrap())::float64().lock().unwrap().as_ref().unwrap()));
}