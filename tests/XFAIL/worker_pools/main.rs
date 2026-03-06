use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
        }
    }

    fn send(&self, val: T) {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            let _ = tx.send(val);
        }
    }

    fn try_send(&self, val: T) -> bool {
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            tx.try_send(val).is_ok()
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        self.rx.lock().unwrap().recv().ok()
    }

    fn try_recv(&self) -> Option<T> {
        self.rx.lock().unwrap().try_recv().ok()
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
        }
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}

pub fn worker(id: Arc<Mutex<Option<i32>>>, jobs: GoChannel<i32>, results: GoChannel<i32>) {
    for j in jobs.clone() {
        println!("{} {} {} {}", "worker".to_string(), (*id.lock().unwrap().as_mut().unwrap()), "started  job".to_string(), j);
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("{} {} {} {}", "worker".to_string(), (*id.lock().unwrap().as_mut().unwrap()), "finished job".to_string(), j);
        results.send(j * 2);
    }
}

fn main() {
    const numJobs: i32 = 5;

    let mut jobs = GoChannel::<i32>::new_buffered(numJobs as usize);
    let mut results = GoChannel::<i32>::new_buffered(numJobs as usize);

    let mut w = Arc::new(Mutex::new(Some(1)));
    while (*w.lock().unwrap().as_mut().unwrap()) <= 3 {
        let jobs_thread = jobs.clone(); let results_thread = results.clone(); let w_thread = Arc::new(Mutex::new(Some((*w.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        worker(w_thread.clone(), jobs_thread.clone(), results_thread.clone());
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut j = Arc::new(Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_mut().unwrap()) <= numJobs {
        jobs.send((*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    jobs.close();

    let mut a = Arc::new(Mutex::new(Some(1)));
    while (*a.lock().unwrap().as_mut().unwrap()) <= numJobs {
        results.recv().unwrap();
        { let mut guard = a.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}