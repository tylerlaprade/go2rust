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

fn main() {
        // Range over slice
    println!("{}", "=== Range over slice ===".to_string());
    let mut numbers = Arc::new(Mutex::new(Some(vec![10, 20, 30, 40, 50])));

        // With index and value
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, num) in __range_values.iter().copied().enumerate() {
        print!("Index {}: {}\n", i, num);
    } }

        // Only value
    println!("{}", "Values only:".to_string());
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for num in __range_values.iter().copied() {
        print!("{} ", num);
    } }
    println!();

        // Only index
    println!("{}", "Indices only:".to_string());
    for i in 0..({ let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        print!("{} ", i);
    }
    println!();

        // Range over array
    println!("{}", "\n=== Range over array ===".to_string());
    let mut arr = Arc::new(Mutex::new(Some(["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()])));
    { let __range_holder = arr.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, fruit) in __range_values.iter().enumerate() {
        print!("{}: {}\n", i, fruit);
    } }

        // Range over string
    println!("{}", "\n=== Range over string ===".to_string());
    let mut text = Arc::new(Mutex::new(Some("Hello, \u{4e16}\u{754c}".to_string())));
    for (i, char) in (*text.lock().unwrap().as_ref().unwrap()).char_indices() {
        print!("Byte {}: {} (Unicode: U+{:04X})\n", i, char, char as u32);
    }

        // Range over map
    println!("{}", "\n=== Range over map ===".to_string());
    let mut ages = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i32>>>>::from([("Alice".to_string(), Arc::new(Mutex::new(Some(25)))), ("Bob".to_string(), Arc::new(Mutex::new(Some(30)))), ("Charlie".to_string(), Arc::new(Mutex::new(Some(35))))]))));

    let mut sortedNames: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (name, _) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        { let __append_target = sortedNames.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(name.clone()); __append_target.clone() };
    }
    (*sortedNames.lock().unwrap().as_mut().unwrap()).sort();
    { let __range_holder = sortedNames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for name in __range_values.iter() {
        print!("{} is {} years old\n", name, { let __map = { let __map_holder = ages.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(name).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
    } }

        // Only keys
    println!("{}", "Keys only:".to_string());
    { let __range_holder = sortedNames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();

        // Range over channel
    println!("{}", "\n=== Range over channel ===".to_string());
    let mut ch = GoChannel::<i32>::new_buffered(5 as usize);

        // Send some values
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        ch.send({ let __bin_i = (*i.lock().unwrap().as_ref().unwrap()).clone(); __bin_i * __bin_i });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    ch.close();

        // Range over closed channel
    for value in ch.clone() {
        print!("Received: {}\n", value);
    }

        // Range with break and continue
    println!("{}", "\n=== Range with break/continue ===".to_string());
    let mut data = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Even numbers only (with continue):".to_string());
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for num in __range_values.iter().copied() {
        if { let __tmp_x = { let __tmp_x = num; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        continue
    }
        print!("{} ", num);
    } }
    println!();

    println!("{}", "Numbers until 6 (with break):".to_string());
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for num in __range_values.iter().copied() {
        if { let __tmp_x = num; let __tmp_y = 6; __tmp_x > __tmp_y } {
        break
    }
        print!("{} ", num);
    } }
    println!();

        // Nested range loops
    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = Arc::new(Mutex::new(Some(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])));

    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, row) in __range_values.iter().enumerate() {
        for (j, val) in row.iter().copied().enumerate() {
        print!("matrix[{}][{}] = {}\n", i, j, val);
    }
    } }

        // Range over empty collections
    println!("{}", "\n=== Range over empty collections ===".to_string());
    let mut emptySlice: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let mut emptyMap: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i32>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));

    println!("{}", "Empty slice:".to_string());
    { let __range_holder = emptySlice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, v) in __range_values.iter().copied().enumerate() {
        print!("This won't print: {}, {}\n", i, v);
    } }
    println!("{}", "Empty slice range completed".to_string());

    println!("{}", "Empty map:".to_string());
    for (k, v) in (*emptyMap.lock().unwrap().as_ref().unwrap()).clone() {
        print!("This won't print: {}, {}\n", k, (*v.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "Empty map range completed".to_string());
}