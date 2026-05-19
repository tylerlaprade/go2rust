use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        if let Some(ref tx) = *self.tx.lock().unwrap() {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}

#[derive(Clone, Debug, Default)]
struct GoTime {
    seconds: i64,
    nanos: i32,
}

fn go_time_civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, m as u32, d as u32)
}

impl GoTime {
    fn now() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        GoTime {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    fn from_unix(seconds: i64, nanos: i64) -> Self {
        let seconds = seconds + nanos.div_euclid(1_000_000_000);
        let nanos = nanos.rem_euclid(1_000_000_000);
        GoTime {
            seconds,
            nanos: nanos as i32,
        }
    }

    fn add(&self, duration: Arc<Mutex<Option<std::time::Duration>>>) -> Arc<Mutex<Option<GoTime>>> {
        let duration = *duration.lock().unwrap().as_ref().unwrap();
        Arc::new(Mutex::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Arc<Mutex<Option<GoTime>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }

    fn unix(&self) -> Arc<Mutex<Option<i64>>> {
        Arc::new(Mutex::new(Some(self.seconds)))
    }

    fn unix_nano(&self) -> Arc<Mutex<Option<i64>>> {
        Arc::new(Mutex::new(Some(
            self.seconds * 1_000_000_000 + self.nanos as i64,
        )))
    }

    fn is_zero(&self) -> Arc<Mutex<Option<bool>>> {
        Arc::new(Mutex::new(Some(self.seconds == 0 && self.nanos == 0)))
    }

    fn format(&self, _layout: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(self.to_string())))
    }
}

impl std::fmt::Display for GoTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let days = self.seconds.div_euclid(86_400);
        let secs_of_day = self.seconds.rem_euclid(86_400);
        let (year, month, day) = go_time_civil_from_days(days);
        let hour = secs_of_day / 3_600;
        let minute = (secs_of_day % 3_600) / 60;
        let second = secs_of_day % 60;
        if self.nanos == 0 {
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} +0000 UTC",
                year, month, day, hour, minute, second
            )
        } else {
            let mut fraction = format!("{:09}", self.nanos);
            while fraction.ends_with('0') {
                fraction.pop();
            }
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{} +0000 UTC",
                year, month, day, hour, minute, second, fraction
            )
        }
    }
}

fn go_channel_after(duration: std::time::Duration) -> GoChannel<GoTime> {
    let channel = GoChannel::<GoTime>::new_buffered(1);
    let thread_channel = channel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        thread_channel.send(GoTime::now());
        thread_channel.close();
    });
    channel
}

pub fn basic_select() {
    let mut ch1 = GoChannel::<String>::new();
    let mut ch2 = GoChannel::<String>::new();

    let ch1_thread = ch1.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));;
        ch1_thread.send("from ch1".to_string());;;
    });

    let ch2_thread = ch2.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(200));;
        ch2_thread.send("from ch2".to_string());;;
    });

    loop {
        if let Some(msg1) = ch1.try_recv() {
            let mut msg1 = Arc::new(Mutex::new(Some(msg1)));
            println!("{} {}", "Received:".to_string(), { let __v = (*msg1.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(msg2) = ch2.try_recv() {
            let mut msg2 = Arc::new(Mutex::new(Some(msg2)));
            println!("{} {}", "Received:".to_string(), { let __v = (*msg2.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

pub fn select_with_timeout() {
    let mut ch = GoChannel::<String>::new();
    let mut timeout = go_channel_after(std::time::Duration::from_millis(100));

    let ch_thread = ch.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));;
        ch_thread.send("delayed message".to_string());;;
    });

    loop {
        if let Some(msg) = ch.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "Got message:".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = timeout.try_recv() {
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
            println!("{} {}", "Received:".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        println!("{}", "No message available".to_string());
        break;
    }

        // Try to receive again (should hit default)
    loop {
        if let Some(msg) = ch.try_recv() {
            let mut msg = Arc::new(Mutex::new(Some(msg)));
            println!("{} {}", "Received:".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v });
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

    let ch1_thread = ch1.clone(); let ch2_thread = ch2.clone(); let quit_thread = quit.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));;
        ch1_thread.send(0);;
        std::thread::sleep(std::time::Duration::from_millis(50));;
        ch2_thread.send(10);;
        std::thread::sleep(std::time::Duration::from_millis(50));;
        ch1_thread.send(1);;
        std::thread::sleep(std::time::Duration::from_millis(50));;
        ch2_thread.send(11);;
        std::thread::sleep(std::time::Duration::from_millis(50));;
        ch1_thread.send(2);;
        std::thread::sleep(std::time::Duration::from_millis(50));;
        ch2_thread.send(12);;
        std::thread::sleep(std::time::Duration::from_millis(50));;
        quit_thread.send(true);;;
    });

    println!("{}", "Starting select loop:".to_string());
    loop {
        loop {
        if let Some(val1) = ch1.try_recv() {
            let mut val1 = Arc::new(Mutex::new(Some(val1)));
            print!("From ch1: {}\n", { let __v = (*val1.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(val2) = ch2.try_recv() {
            let mut val2 = Arc::new(Mutex::new(Some(val2)));
            print!("From ch2: {}\n", { let __v = (*val2.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = quit.try_recv() {
            println!("{}", "Quit signal received".to_string());
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
}

pub fn select_with_send() {
    let mut ch1 = GoChannel::<String>::new_buffered(1 as usize);
    let mut ch2 = GoChannel::<String>::new_buffered(1 as usize);
    ch2.send("preloaded ch2".to_string());

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
            println!("{} {}", "Reading from ch2:".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v });
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