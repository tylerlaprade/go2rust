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

pub fn sender(ch: GoChannel<i32>) {
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        print!("Sending: {}\n", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v });
        ch.send((*i.lock().unwrap().as_ref().unwrap()));
        std::thread::sleep(std::time::Duration::from_millis(100));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    ch.close();
}

pub fn receiver(ch: GoChannel<i32>) {
    while true {
        let (mut value, mut ok) = match ch.recv() { Some(v) => (Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(Default::default()))), Arc::new(Mutex::new(Some(false)))) };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        println!("{}", "Channel closed".to_string());
        break
    }
        print!("Received: {}\n", { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
}

fn main() {
        // Unbuffered channel
    let mut ch = GoChannel::<i32>::new();

    let ch_thread = ch.clone(); std::thread::spawn(move || {
        sender(ch_thread.clone());
    });
    let ch_thread = ch.clone(); std::thread::spawn(move || {
        receiver(ch_thread.clone());
    });

    std::thread::sleep(std::time::Duration::from_millis(500));

        // Buffered channel
    let mut buffered = GoChannel::<String>::new_buffered(3 as usize);
    buffered.send("first".to_string());
    buffered.send("second".to_string());
    buffered.send("third".to_string());

    println!("{}", "Buffered channel contents:".to_string());
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        let mut msg = Arc::new(Mutex::new(Some(buffered.recv().unwrap())));
        println!("{} {}", "Got:".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Channel range
    let mut numbers = GoChannel::<i32>::new_buffered(5 as usize);
    let numbers_closure_clone = numbers.clone(); let numbers_thread = numbers.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(10)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 15; __tmp_x < __tmp_y } {
        numbers_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        numbers_thread.close();;;
    });

    println!("{}", "Range over channel:".to_string());
    for num in numbers.clone() {
        println!("{} {}", "Number:".to_string(), num);
    }
}