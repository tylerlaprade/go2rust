use std::sync::{Arc, Mutex};

fn main() {
    rand.seed(Arc::new(Mutex::new(Some(time.now().unix_nano()))));
    println!("{} {}", "Random int:".to_string(), (*rand.intn(Arc::new(Mutex::new(Some(100)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Random float:".to_string(), (*rand.float64().lock().unwrap().as_ref().unwrap()));
}