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

pub fn basic_select() {
    let mut ch1 = GoChannel::<String>::new();
    let mut ch2 = GoChannel::<String>::new();

    let ch1_closure_clone = ch1.clone(); let ch1_thread = ch1.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));;
        ch1_thread.send("from ch1".to_string());;;
    });

    let ch2_closure_clone = ch2.clone(); let ch2_thread = ch2.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(200));;
        ch2_thread.send("from ch2".to_string());;;
    });

    loop {
        if let Some(msg1) = ch1.try_recv() {
            let mut msg1 = Arc::new(Mutex::new(Some(msg1)));
            println!("{} {}", "Received:".to_string(), (*msg1.lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(msg2) = ch2.try_recv() {
            let mut msg2 = Arc::new(Mutex::new(Some(msg2)));
            println!("{} {}", "Received:".to_string(), (*msg2.lock().unwrap().as_ref().unwrap()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

pub fn select_with_timeout() {
    let mut ch = GoChannel::<String>::new();

    let ch_closure_clone = ch.clone(); let ch_thread = ch.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));;
        ch_thread.send("delayed message".to_string());;;
    });

    loop {
        if let Some(msg) = ch.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "Got message:".to_string(), (*msg.lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(100)).try_recv() {
            println!("{}", "Timeout occurred".to_string());
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

pub fn select_with_default() {
    let mut ch = GoChannel::<String>::new_buffered(1 as usize);

        // Non-blocking send
    loop {
        if ch.try_send("hello".to_string()) {
            println!("{}", "Sent message".to_string());
            break;
        }
        println!("{}", "Channel full, couldn't send".to_string());
        break;
    }

        // Non-blocking receive
    loop {
        if let Some(msg) = ch.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "Received:".to_string(), (*msg.lock().unwrap().as_ref().unwrap()));
            break;
        }
        println!("{}", "No message available".to_string());
        break;
    }

        // Try to receive again (should hit default)
    loop {
        if let Some(msg) = ch.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "Received:".to_string(), (*msg.lock().unwrap().as_ref().unwrap()));
            break;
        }
        println!("{}", "No message available".to_string());
        break;
    }
}

pub fn select_loop() {
    let mut ch1 = GoChannel::<i32>::new();
    let mut ch2 = GoChannel::<i32>::new();
    let mut quit = GoChannel::<bool>::new();

    let ch1_closure_clone = ch1.clone(); let ch1_thread = ch1.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        ch1_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        std::thread::sleep(std::time::Duration::from_millis(100));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };;
    });

    let ch2_closure_clone = ch2.clone(); let ch2_thread = ch2.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(10)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 13 {
        ch2_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        std::thread::sleep(std::time::Duration::from_millis(150));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };;
    });

    let quit_closure_clone = quit.clone(); let quit_thread = quit.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));;
        quit_thread.send(true);;;
    });

    println!("{}", "Starting select loop:".to_string());
    while true {
        loop {
        if let Some(val1) = ch1.try_recv() {
            let mut val1 = Arc::new(Mutex::new(Some(val1)));
            print!("From ch1: {}\n", (*val1.lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(val2) = ch2.try_recv() {
            let mut val2 = Arc::new(Mutex::new(Some(val2)));
            print!("From ch2: {}\n", (*val2.lock().unwrap().as_ref().unwrap()));
            break;
        }
        if let Some(_) = quit.try_recv() {
            println!("{}", "Quit signal received".to_string());
            return;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
}

pub fn select_with_send() {
    let mut ch1 = GoChannel::<String>::new_buffered(1 as usize);
    let mut ch2 = GoChannel::<String>::new_buffered(1 as usize);

    loop {
        if ch1.try_send("message to ch1".to_string()) {
            println!("{}", "Sent to ch1".to_string());
            break;
        }
        if ch2.try_send("message to ch2".to_string()) {
            println!("{}", "Sent to ch2".to_string());
            break;
        }
        println!("{}", "Both channels busy".to_string());
        break;
    }

        // Read from both channels
    println!("{} {}", "Reading from ch1:".to_string(), ch1.recv().unwrap());

    loop {
        if let Some(msg) = ch2.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "Reading from ch2:".to_string(), (*msg.lock().unwrap().as_ref().unwrap()));
            break;
        }
        println!("{}", "ch2 is empty".to_string());
        break;
    }
}

fn main() {
    println!("{}", "=== Basic select ===".to_string());
    basic_select();

    println!("{}", "\n=== Select with timeout ===".to_string());
    select_with_timeout();

    println!("{}", "\n=== Select with default ===".to_string());
    select_with_default();

    println!("{}", "\n=== Select with send ===".to_string());
    select_with_send();

    println!("{}", "\n=== Select loop ===".to_string());
    select_loop();

    println!("{}", "\n=== All examples completed ===".to_string());
}