use std::sync::{Arc, Mutex};
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
                |current| current.checked_sub(1),
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
                |current| current.checked_sub(1),
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

#[derive(Clone, Debug)]
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

#[derive(Clone)]
struct GoContext {
    done: GoChannel<bool>,
    err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    label: String,
}

type GoCancelFunc = std::sync::Arc<dyn Fn() + Send + Sync>;
type GoCancelCauseFunc = Box<dyn FnMut(Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) -> () + Send + Sync>;

impl GoContext {
    fn background() -> GoContext {
        GoContext {
            done: GoChannel::<bool>::new(),
            err: std::sync::Arc::new(std::sync::Mutex::new(None)),
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            label: "context.Background".to_string(),
        }
    }

    fn parent_label(parent: &Arc<Mutex<Option<GoContext>>>) -> String {
        parent
            .lock()
            .unwrap()
            .as_ref()
            .map(|ctx| ctx.label.clone())
            .unwrap_or_else(|| "context.Context".to_string())
    }

    fn with_timeout(parent: Arc<Mutex<Option<GoContext>>>, duration: std::time::Duration) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelFunc>>>) {
        let label = format!("{}.WithDeadline", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let timeout_done = done.clone();
        let timeout_err = err.clone();
        let timeout_cancelled = cancelled.clone();
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            if !timeout_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *timeout_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context deadline exceeded".to_string()));
                timeout_done.send(true);
                timeout_done.close();
            }
        });

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelFunc = std::sync::Arc::new(move || {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn with_cancel(parent: Arc<Mutex<Option<GoContext>>>) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelFunc>>>) {
        let label = format!("{}.WithCancel", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelFunc = std::sync::Arc::new(move || {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn with_cancel_cause(parent: Arc<Mutex<Option<GoContext>>>) -> (Arc<Mutex<Option<GoContext>>>, Arc<Mutex<Option<GoCancelCauseFunc>>>) {
        let label = format!("{}.WithCancelCause", GoContext::parent_label(&parent));
        let done = GoChannel::<bool>::new_buffered(1);
        let err = std::sync::Arc::new(std::sync::Mutex::new(None::<Box<dyn std::error::Error + Send + Sync>>));
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let context = GoContext {
            done: done.clone(),
            err: err.clone(),
            cancelled: cancelled.clone(),
            label,
        };

        let cancel_done = done.clone();
        let cancel_err = err.clone();
        let cancel_cancelled = cancelled.clone();
        let cancel: GoCancelCauseFunc = Box::new(move |_cause| {
            if !cancel_cancelled.swap(true, std::sync::atomic::Ordering::SeqCst) {
                *cancel_err.lock().unwrap() = Some(Box::<dyn std::error::Error + Send + Sync>::from("context canceled".to_string()));
                cancel_done.send(true);
                cancel_done.close();
            }
        });

        (
            Arc::new(Mutex::new(Some(context))),
            Arc::new(Mutex::new(Some(cancel))),
        )
    }

    fn done(&self) -> GoChannel<bool> {
        self.done.clone()
    }

    fn err(&self) -> Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> {
        self.err.clone()
    }
}

impl std::fmt::Display for GoContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl std::fmt::Debug for GoContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut ctx, mut cancel) = GoContext::with_timeout(Arc::new(Mutex::new(Some(GoContext::background()))).clone(), std::time::Duration::from_secs(1));
    let cancel_defer_captured = cancel.clone(); __defer_stack.push(Box::new(move || {
        { let __f_ptr: *mut GoCancelFunc = { let mut __f_guard = cancel_defer_captured.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut GoCancelFunc }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

    let mut operationDone = go_channel_after(std::time::Duration::from_millis(500));

    loop {
        if let Some(_) = operationDone.try_recv() {
            println!("{}", "Operation completed".to_string());
            break;
        }
        if let Some(_) = (*ctx.lock().unwrap().as_ref().unwrap()).done().try_recv() {
            println!("{} {}", "Context cancelled:".to_string(), format!("{}", (*((*ctx.lock().unwrap().as_ref().unwrap()).err()).lock().unwrap().as_ref().unwrap())));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    let (mut ctx2, mut cancel2) = GoContext::with_cancel_cause(Arc::new(Mutex::new(Some(GoContext::background()))).clone());
    { let __f_ptr: *mut GoCancelCauseFunc = { let mut __f_guard = cancel2.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut GoCancelCauseFunc }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("boom".to_string()))))) };
    (*ctx2.lock().unwrap().as_ref().unwrap()).done().recv().unwrap();
    println!("{} {}", "Cause cancel:".to_string(), format!("{}", (*((*ctx2.lock().unwrap().as_ref().unwrap()).err()).lock().unwrap().as_ref().unwrap())));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}