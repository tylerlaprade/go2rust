use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

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
        // Nested loops with labels
    println!("{}", "=== Nested loops with labels ===".to_string());

    let mut i = Arc::new(Mutex::new(Some(1)));
    'outer: while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } {
        print!("Breaking outer loop at i={}, j={}\n", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v });
        break 'outer
    }
        print!("i={}, j={}\n", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Continue with labels
    println!("{}", "\n=== Continue with labels ===".to_string());

    let mut i = Arc::new(Mutex::new(Some(1)));
    'outer_continue: while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } {
        print!("Continuing outer loop at i={}, j={}\n", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer_continue
    }
        print!("i={}, j={}\n", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Complex switch with fallthrough
    println!("{}", "\n=== Complex switch with fallthrough ===".to_string());

    let mut num = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        print!("Number {}: ", { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v });
        {
        let _switch_val = { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 1) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            print!("{}", "One".to_string());
            _fallthrough = true;
        }
        if !_matched && (_switch_val == 2) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            print!("{}", " Two-ish".to_string());
        }
        if !_matched && (_switch_val == 3) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            print!("{}", "Three".to_string());
        }
        if !_matched && (_switch_val == 4 || _switch_val == 5) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            print!("{}", " Four-or-Five".to_string());
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            print!("{}", " Other".to_string());
        }
    }
        println!();
        { let mut guard = num.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Nested switch statements
    println!("{}", "\n=== Nested switch statements ===".to_string());

    let mut category = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*category.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x <= __tmp_y } {
        let mut item = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*item.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x <= __tmp_y } {
        print!("Category {}, Item {}: ", { let __v = (*category.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*item.lock().unwrap().as_ref().unwrap()).clone(); __v });

        { let _switch_val = { let __v = (*category.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            { let _switch_val = { let __v = (*item.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            println!("{}", "Electronics - Phone".to_string());
        } else if _switch_val == (2) {
            println!("{}", "Electronics - Laptop".to_string());
        }
    }
        } else if _switch_val == (2) {
            { let _switch_val = { let __v = (*item.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            println!("{}", "Books - Fiction".to_string());
        } else if _switch_val == (2) {
            println!("{}", "Books - Non-fiction".to_string());
        }
    }
        }
    }
        { let mut guard = item.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = category.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Complex for loop conditions
    println!("{}", "\n=== Complex for loop conditions ===".to_string());

        // Multiple variables in for loop
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(10))));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        print!("i={}, j={}, sum={}\n", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x >= __tmp_y } {
        break
    }
        { let __tmp_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_1 = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; *i.lock().unwrap() = Some(__tmp_0); *j.lock().unwrap() = Some(__tmp_1); };
    }

        // For loop with complex condition
    println!("{}", "\n=== For loop with complex condition ===".to_string());

    let (mut x, mut y) = (Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(1))));
    while { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = 100; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        print!("x={}, y={}, product={}\n", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y });
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 2); };
    } else {
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
    }
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Complex if-else chains
    println!("{}", "\n=== Complex if-else chains ===".to_string());

    let mut score = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x <= __tmp_y } {
        let mut grade: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut message: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

        if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 90; __tmp_x >= __tmp_y } {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 95; __tmp_x >= __tmp_y } {
        { let new_val = "Excellent!".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "Great job!".to_string(); *message.lock().unwrap() = Some(new_val); };
    }
    } else if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 80; __tmp_x >= __tmp_y } {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 85; __tmp_x >= __tmp_y } {
        { let new_val = "Good work!".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "Not bad!".to_string(); *message.lock().unwrap() = Some(new_val); };
    }
    } else if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 70; __tmp_x >= __tmp_y } {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Average".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x >= __tmp_y } {
        { let new_val = "D".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Below average".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Needs improvement".to_string(); *message.lock().unwrap() = Some(new_val); };
    }

        print!("Score {}: Grade {} - {}\n", { let __v = (*score.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*grade.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*message.lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let mut guard = score.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 25); };
    }

        // Range with complex break/continue logic
    println!("{}", "\n=== Range with complex break/continue ===".to_string());

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Processing numbers:".to_string());
    { let __range_guard = numbers.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, num) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = { let __tmp_x = num; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __tmp_x = num; let __tmp_y = 6; __tmp_x > __tmp_y } {
        print!("Stopping at even number {} (index {})\n", num, i);
        break
    }
        print!("Skipping even number {} (index {})\n", num, i);
        continue
    }
        if { let __tmp_x = num; let __tmp_y = 7; __tmp_x == __tmp_y } {
        print!("Found lucky number {} at index {}\n", num, i);
        continue
    }
        print!("Processing odd number {} (index {})\n", num, i);
    } }

        // Nested range loops
    println!("{}", "\n=== Nested range loops ===".to_string());

    let mut matrix = Arc::new(Mutex::new(Some(vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["d".to_string(), "e".to_string(), "f".to_string()], vec!["g".to_string(), "h".to_string(), "i".to_string()]])));

    { let __range_guard = matrix.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (rowIdx, row) in __range_values.iter().enumerate() {
        for (colIdx, cell) in row.iter().enumerate() {
        if { let __tmp_x = cell; let __tmp_y = "e"; __tmp_x == __tmp_y } {
        print!("Found center at [{}][{}]: {}\n", rowIdx, colIdx, cell);
        continue
    }
        if { let __tmp_x = rowIdx; let __tmp_y = 2; __tmp_x == __tmp_y } && { let __tmp_x = colIdx; let __tmp_y = 2; __tmp_x == __tmp_y } {
        print!("Last cell [{}][{}]: {}\n", rowIdx, colIdx, cell);
        break
    }
        print!("[{}][{}]: {} ", rowIdx, colIdx, cell);
    }
        println!();
    } }

        // Select with complex channel operations
    println!("{}", "\n=== Select with complex channel operations ===".to_string());

    let mut ch1 = GoChannel::<i32>::new_buffered(2 as usize);
    let mut ch2 = GoChannel::<String>::new_buffered(2 as usize);

    ch1.send(1);
    loop {
        if let Some(val) = ch1.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received int: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(val) = ch2.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received string: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    ch2.send("hello".to_string());
    loop {
        if let Some(val) = ch1.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received int: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(val) = ch2.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received string: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    ch2.send("world".to_string());
    loop {
        if let Some(val) = ch1.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received int: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(val) = ch2.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received string: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    ch1.send(2);
    loop {
        if let Some(val) = ch1.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received int: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(val) = ch2.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received string: {}\n", { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    println!("{}", "Channel processing complete".to_string());

        // Complex error handling flow
    println!("{}", "\n=== Complex error handling flow ===".to_string());

    let mut processData = Arc::new(Mutex::new(Some(Box::new(move |data: Arc<Mutex<Option<Vec<i32>>>>| -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*data.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("empty data")))));
    }
        { let __range_guard = data.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, val) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = val; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("negative value at index {}: {}", i, val)))));
    }
        if { let __tmp_x = val; let __tmp_y = 100; __tmp_x > __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("value too large at index {}: {}", i, val)))));
    }
    } }
        return Arc::new(Mutex::new(None));
    }) as Box<dyn Fn(Arc<Mutex<Option<Vec<i32>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>)));

    let mut testData = Arc::new(Mutex::new(Some(vec![vec![1, 2, 3], vec![], vec![1, -2, 3], vec![1, 200, 3], vec![10, 20, 30]])));

    { let __range_guard = testData.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, data) in __range_values.iter().enumerate() {
        print!("Testing dataset {}: {}\n", { let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }, format_slice_values(data));
        let mut err = { let __f_guard = processData.lock().unwrap(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Arc::new(Mutex::new(Some(data.clone())))) };
    if (*err.lock().unwrap()).is_some() {
        print!("  Error: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        continue
    }
        print!("  Success: data is valid\n");
    } }
}