use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


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

#[derive(Debug, Clone, Default)]
pub struct holder {
    pub ctx: Arc<Mutex<Option<GoContext>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { ctx: self.ctx.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.ctx.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let _ = Arc::new(Mutex::new(Some(format!("{}", holder { ctx: Arc::new(Mutex::new(Some(GoContext::background()))), ..Default::default() }))));
    println!("{}", format!("{}", (*(Arc::new(Mutex::new(Some(GoContext::background())))).lock().unwrap().as_ref().unwrap())));
}