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
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
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

/// Function with anonymous struct parameter
pub fn print_person(p: Arc<Mutex<Option<AnonymousStruct1>>>) {
    print!("Person: {} is {} years old\n", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).age.clone(); __field }.lock().unwrap().as_ref().unwrap()));
}

/// Function returning anonymous struct
pub fn get_point() -> Arc<Mutex<Option<AnonymousStruct2>>> {

    return Arc::new(Mutex::new(Some(AnonymousStruct2 { x: Arc::new(Mutex::new(Some(10 as i32))), y: Arc::new(Mutex::new(Some(20 as i32))) })));
}

/// Function with multiple anonymous struct parameters
pub fn compare_points(p1: Arc<Mutex<Option<AnonymousStruct2>>>, p2: Arc<Mutex<Option<AnonymousStruct2>>>) -> Arc<Mutex<Option<bool>>> {

    return Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*p2.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y })));
}

/// Function returning multiple values including anonymous struct
pub fn get_config() -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<AnonymousStruct3>>>) {

    return (Arc::new(Mutex::new(Some("server".to_string()))), Arc::new(Mutex::new(Some(AnonymousStruct3 { port: Arc::new(Mutex::new(Some(8080 as i32))), timeout: Arc::new(Mutex::new(Some(30 as i32))) }))));
}

/// Function with anonymous struct pointer parameter
pub fn update_settings(s: Arc<Mutex<Option<AnonymousStruct4>>>) {
    { let new_val = true; *(*s.lock().unwrap().as_ref().unwrap()).debug.lock().unwrap() = Some(new_val); };
    { let new_val = true; *(*s.lock().unwrap().as_ref().unwrap()).verbose.lock().unwrap() = Some(new_val); };
}

/// Function with anonymous struct in channel
pub fn process_events(ch: GoChannel<AnonymousStruct5>) {
    for event in ch.clone() {
        print!("Event [{}]: {}\n", (*event.r#type.lock().unwrap().as_ref().unwrap()).clone(), (*event.message.lock().unwrap().as_ref().unwrap()).clone());
    }
}

fn main() {
        // Test function with anonymous struct parameter
    print_person(Arc::new(Mutex::new(Some(AnonymousStruct1 { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30 as i32))) }))));

        // Test function returning anonymous struct
    let mut point = get_point();
    print!("Point: ({}, {})\n", (*{ let __field = (*point.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()), (*{ let __field = (*point.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()));

        // Test function with multiple anonymous struct parameters
    let mut p1 = Arc::new(Mutex::new(Some(AnonymousStruct2 { x: Arc::new(Mutex::new(Some(5 as i32))), y: Arc::new(Mutex::new(Some(10 as i32))) })));
    let mut p2 = Arc::new(Mutex::new(Some(AnonymousStruct2 { x: Arc::new(Mutex::new(Some(5 as i32))), y: Arc::new(Mutex::new(Some(10 as i32))) })));
    let mut p3 = Arc::new(Mutex::new(Some(AnonymousStruct2 { x: Arc::new(Mutex::new(Some(10 as i32))), y: Arc::new(Mutex::new(Some(20 as i32))) })));
    print!("p1 == p2: {}\n", (*compare_points(p1.clone(), p2.clone()).lock().unwrap().as_ref().unwrap()));
    print!("p1 == p3: {}\n", (*compare_points(p1.clone(), p3.clone()).lock().unwrap().as_ref().unwrap()));

        // Test function returning multiple values including anonymous struct
    let (mut name, mut config) = get_config();
    print!("Config for {}: Port={}, Timeout={}\n", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*{ let __field = (*config.lock().unwrap().as_ref().unwrap()).port.clone(); __field }.lock().unwrap().as_ref().unwrap()), (*{ let __field = (*config.lock().unwrap().as_ref().unwrap()).timeout.clone(); __field }.lock().unwrap().as_ref().unwrap()));

        // Test function with anonymous struct pointer
    let mut settings = Arc::new(Mutex::new(Some(AnonymousStruct4 { debug: Arc::new(Mutex::new(Some(false))), verbose: Arc::new(Mutex::new(Some(false))) })));
    print!("Settings before: Debug={}, Verbose={}\n", (*{ let __field = (*settings.lock().unwrap().as_ref().unwrap()).debug.clone(); __field }.lock().unwrap().as_ref().unwrap()), (*{ let __field = (*settings.lock().unwrap().as_ref().unwrap()).verbose.clone(); __field }.lock().unwrap().as_ref().unwrap()));
    update_settings(settings.clone());
    print!("Settings after: Debug={}, Verbose={}\n", (*{ let __field = (*settings.lock().unwrap().as_ref().unwrap()).debug.clone(); __field }.lock().unwrap().as_ref().unwrap()), (*{ let __field = (*settings.lock().unwrap().as_ref().unwrap()).verbose.clone(); __field }.lock().unwrap().as_ref().unwrap()));

        // Test function with anonymous struct in channel
    let mut eventCh = GoChannel::<AnonymousStruct5>::new_buffered(2 as usize);
    eventCh.send(AnonymousStruct5 { r#type: Arc::new(Mutex::new(Some("info".to_string()))), message: Arc::new(Mutex::new(Some("System started".to_string()))) });
    eventCh.send(AnonymousStruct5 { r#type: Arc::new(Mutex::new(Some("error".to_string()))), message: Arc::new(Mutex::new(Some("Connection failed".to_string()))) });
    eventCh.close();
    process_events(eventCh.clone());
}

#[derive(Debug, Clone)]
struct AnonymousStruct1 {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, age: { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), age: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.age.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct2 {
    x: Arc<Mutex<Option<i32>>>,
    y: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, y: { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { x: Arc::new(Mutex::new(Some(0))), y: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.lock().unwrap().as_ref().unwrap()), (*self.y.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct3 {
    port: Arc<Mutex<Option<i32>>>,
    timeout: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { port: { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, timeout: { let __guard = self.timeout.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { port: Arc::new(Mutex::new(Some(0))), timeout: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.port.lock().unwrap().as_ref().unwrap()), (*self.timeout.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct4 {
    debug: Arc<Mutex<Option<bool>>>,
    verbose: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { debug: { let __guard = self.debug.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, verbose: { let __guard = self.verbose.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        Self { debug: Arc::new(Mutex::new(Some(false))), verbose: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.debug.lock().unwrap().as_ref().unwrap()), (*self.verbose.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
struct AnonymousStruct5 {
    r#type: Arc<Mutex<Option<String>>>,
    message: Arc<Mutex<Option<String>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, message: { let __guard = self.message.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(Some(String::new()))), message: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.message.lock().unwrap().as_ref().unwrap()))
    }
}
