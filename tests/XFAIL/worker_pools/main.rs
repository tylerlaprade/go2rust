use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn worker(id: Arc<Mutex<Option<i32>>>, jobs: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>, results: Arc<Mutex<Option</* TODO: Unhandled type *ast.ChanType */ Arc<Mutex<Option<()>>>>>>) {
    for j in 0..(*jobs.lock().unwrap().as_mut().unwrap()).len() {
        println!("{} {} {} {}", "worker".to_string(), (*id.lock().unwrap().as_mut().unwrap()), "started  job".to_string(), j);
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("{} {} {} {}", "worker".to_string(), (*id.lock().unwrap().as_mut().unwrap()), "finished job".to_string(), j);
        // TODO: Unhandled statement type: SendStmt
    }
}

fn main() {
    const numJobs: i32 = 5;

    let mut jobs = ;
    let mut results = ;

    let mut w = Arc::new(Mutex::new(Some(1)));
    while (*w.lock().unwrap().as_mut().unwrap()) <= 3 {
        std::thread::spawn(move || {
        worker(w.clone(), jobs.clone(), results.clone());
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut j = Arc::new(Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_mut().unwrap()) <= numJobs {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*close.lock().unwrap().as_ref().unwrap())(jobs.clone());

    let mut a = Arc::new(Mutex::new(Some(1)));
    while (*a.lock().unwrap().as_mut().unwrap()) <= numJobs {
        <-(*results.lock().unwrap().as_mut().unwrap());
        { let mut guard = a.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}