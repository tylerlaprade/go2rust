use std::sync::{Arc, Mutex};
use std::thread;


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
    let _ = { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v };
    for j in jobs.clone() {
        results.send({ let __tmp_x = j; let __tmp_y = 2; __tmp_x * __tmp_y });
    }
}

fn main() {
    const numJobs: i32 = 5;

    let mut jobs = GoChannel::<i32>::new_buffered(numJobs as usize);
    let mut results = GoChannel::<i32>::new_buffered(numJobs as usize);

    let mut w = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        let jobs_thread = jobs.clone(); let results_thread = results.clone(); let w_thread = Arc::new(Mutex::new(Some((*w.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        worker(Arc::new(Mutex::new(Some((*w_thread.lock().unwrap().as_ref().unwrap()).clone()))), jobs_thread.clone(), results_thread.clone());
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = numJobs; __tmp_x <= __tmp_y } {
        jobs.send((*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    jobs.close();

    let mut total = Arc::new(Mutex::new(Some(0)));
    let mut a = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = numJobs; __tmp_x <= __tmp_y } {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + results.recv().unwrap()); };
        { let mut guard = a.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{} {}", "Processed jobs:".to_string(), numJobs);
    println!("{} {}", "Result total:".to_string(), { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
}