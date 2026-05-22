use std::any::Any;
use std::collections::BTreeMap;
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

pub fn multiple_returns() -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<bool>>>) {

    return (Arc::new(Mutex::new(Some(42 as i32))), Arc::new(Mutex::new(Some("hello".to_string()))), Arc::new(Mutex::new(Some(true))));
}

pub fn process_slice(slice: Arc<Mutex<Option<Vec<i32>>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) {
    let mut sum: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut count: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = 0; *sum.lock().unwrap() = Some(new_val); };
    { let new_val = (*slice.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *count.lock().unwrap() = Some(new_val); };
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter().copied() {
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + val); };
    } }
    return (sum, count);
}

fn main() {
        // Ignoring return values
    println!("{}", format!("{}", "=== Ignoring return values ===".to_string()));

        // Ignore all but first return value
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Ignore first and last return values
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Ignore first two return values
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", { let __v = (*flag.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Ignoring in range loops
    println!("{}", format!("{}", "\n=== Ignoring in range loops ===".to_string()));

    let mut slice = Arc::new(Mutex::new(Some(vec![10, 20, 30, 40, 50])));

        // Ignore index, use only value
    println!("{}", format!("{}", "Values only:".to_string()));
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter().copied() {
        print!("{} ", val);
    } }
    println!();

        // Ignore value, use only index
    println!("{}", format!("{}", "Indices only:".to_string()));
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, _) in __range_values.iter().copied().enumerate() {
        print!("{} ", i);
    } }
    println!();

        // Alternative: just use index (more idiomatic)
    println!("{}", format!("{}", "Indices (idiomatic):".to_string()));
    for i in 0..({ let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        print!("{} ", i);
    }
    println!();

        // Ignoring in map iteration
    println!("{}", format!("{}", "\n=== Ignoring in map iteration ===".to_string()));

    let mut ages = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("Alice".to_string(), Arc::new(Mutex::new(Some(25)))), ("Bob".to_string(), Arc::new(Mutex::new(Some(30)))), ("Carol".to_string(), Arc::new(Mutex::new(Some(35))))]))));

        // Ignore values, use only keys (sorted for deterministic output)
    println!("{}", format!("{}", "Keys only:".to_string()));
    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (name, _) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = names.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(name.clone()); __append_target.clone() }; names = new_val; };
    }
    { let mut __sort_guard = names.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();

        // Ignore keys, use only values (sorted for deterministic output)
    println!("{}", format!("{}", "Values only:".to_string()));
    let mut sortedAges: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    for (_, age) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = sortedAges.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*age.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; sortedAges = new_val; };
    }
    { let mut __sort_guard = sortedAges.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = sortedAges.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for age in __range_values.iter().copied() {
        print!("{} ", age);
    } }
    println!();

        // Ignoring function parameters (not applicable in Go, but showing concept)
    println!("{}", format!("{}", "\n=== Ignoring some return values in assignment ===".to_string()));

    let (mut sum, _) = process_slice(slice.clone());
    print!("Sum (ignoring count): {}\n", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v });

    let (_, mut count) = process_slice(slice.clone());
    print!("Count (ignoring sum): {}\n", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Using blank identifier in variable declarations
    println!("{}", format!("{}", "\n=== Blank identifier in declarations ===".to_string()));

        // This would be useful for side effects only
    let _ = "This string is assigned but not used".to_string();

        // Multiple assignments with blank identifier
    let (mut a, _, mut c) = (Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(3))));
    print!("a={}, c={} (middle value ignored)\n", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Blank identifier with type assertion
    println!("{}", format!("{}", "\n=== Blank identifier with type assertion ===".to_string()));

    let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("hello world".to_string()) as Box<dyn Any + Send + Sync>)));

        // Check if it's a string, but don't use the value
    {
        let (_, mut ok) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), Arc::new(Mutex::new(Some(false))))
        }
    });;
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            println!("{}", format!("{}", "Value is a string (but we ignored the actual value)".to_string()));;
        }
    }

        // Check if it's an int, but don't use the value
    {
        let (_, mut ok) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))))
        }
    });;
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            println!("{}", format!("{}", "Value is an int".to_string()));;
        } else {
            println!("{}", format!("{}", "Value is not an int".to_string()));;
        }
    }

        // Blank identifier in channel operations
    println!("{}", format!("{}", "\n=== Blank identifier with channels ===".to_string()));

    let mut ch = GoChannel::<i32>::new_buffered(3 as usize);
    ch.send(1);
    ch.send(2);
    ch.send(3);
    ch.close();

        // Read from channel but ignore the value
    for _ in ch.clone() {
        println!("{}", format!("{}", "Received a value (but ignored it)".to_string()));
    }

        // Blank identifier in error handling
    println!("{}", format!("{}", "\n=== Blank identifier in error handling ===".to_string()));

        // Sometimes you might want to ignore errors (not recommended in real code)
    let (mut result, _) = process_slice(Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5]))));
    print!("Result (ignoring potential error): {}\n", { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v });

    println!("{}", format!("{}", "\n=== Complex example ===".to_string()));

        // Complex example with multiple blank identifiers
    let mut data = Arc::new(Mutex::new(Some(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])));

    let mut total = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for row in __range_values.iter() {
        for val in row.iter().copied() {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + val); };
    }
    } }
        // Ignore column index
    print!("Total of all values: {}\n", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
}