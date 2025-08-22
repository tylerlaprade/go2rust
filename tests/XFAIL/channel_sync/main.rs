use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn worker(done: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    (*fmt.lock().unwrap().as_mut().unwrap()).print(Arc::new(Mutex::new(Some("working...".to_string()))));
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("{}", "done".to_string());

    // TODO: Unhandled statement type: SendStmt
}

fn main() {
    let mut done = ;
    std::thread::spawn(move || {
        worker(done.clone());
    });

    <-(*done.lock().unwrap().as_mut().unwrap());
}