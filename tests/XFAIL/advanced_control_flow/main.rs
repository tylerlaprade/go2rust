use std::error::Error;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};
use std::thread;

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

fn main() {
        // Nested loops with labels
    println!("{}", "=== Nested loops with labels ===".to_string());

    let mut i = Arc::new(Mutex::new(Some(1)));
    'outer: while (*i.lock().unwrap().as_ref().unwrap()) <= 3 {
        let mut j = Arc::new(Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_ref().unwrap()) <= 3 {
        if (*i.lock().unwrap().as_ref().unwrap()) == 2 && (*j.lock().unwrap().as_ref().unwrap()) == 2 {
        print!("Breaking outer loop at i={}, j={}\n", (*i.lock().unwrap().as_ref().unwrap()), (*j.lock().unwrap().as_ref().unwrap()));
        break 'outer
    }
        print!("i={}, j={}\n", (*i.lock().unwrap().as_ref().unwrap()), (*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Continue with labels
    println!("{}", "\n=== Continue with labels ===".to_string());

    let mut i = Arc::new(Mutex::new(Some(1)));
    'outer_continue: while (*i.lock().unwrap().as_ref().unwrap()) <= 3 {
        let mut j = Arc::new(Mutex::new(Some(1)));
    while (*j.lock().unwrap().as_ref().unwrap()) <= 3 {
        if (*j.lock().unwrap().as_ref().unwrap()) == 2 {
        print!("Continuing outer loop at i={}, j={}\n", (*i.lock().unwrap().as_ref().unwrap()), (*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'outer_continue
    }
        print!("i={}, j={}\n", (*i.lock().unwrap().as_ref().unwrap()), (*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Complex switch with fallthrough
    println!("{}", "\n=== Complex switch with fallthrough ===".to_string());

    let mut num = Arc::new(Mutex::new(Some(1)));
    while (*num.lock().unwrap().as_ref().unwrap()) <= 5 {
        print!("Number {}: ", (*num.lock().unwrap().as_ref().unwrap()));
        {
        let _switch_val = (*num.lock().unwrap().as_ref().unwrap());
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
    while (*category.lock().unwrap().as_ref().unwrap()) <= 2 {
        let mut item = Arc::new(Mutex::new(Some(1)));
    while (*item.lock().unwrap().as_ref().unwrap()) <= 2 {
        print!("Category {}, Item {}: ", (*category.lock().unwrap().as_ref().unwrap()), (*item.lock().unwrap().as_ref().unwrap()));

        { let _switch_val = (*category.lock().unwrap().as_ref().unwrap());
    match _switch_val {
        1 => {
            { let _switch_val = (*item.lock().unwrap().as_ref().unwrap());
    match _switch_val {
        1 => {
            println!("{}", "Electronics - Phone".to_string());
        }
        2 => {
            println!("{}", "Electronics - Laptop".to_string());
        }
        _ => {}
    } }
        }
        2 => {
            { let _switch_val = (*item.lock().unwrap().as_ref().unwrap());
    match _switch_val {
        1 => {
            println!("{}", "Books - Fiction".to_string());
        }
        2 => {
            println!("{}", "Books - Non-fiction".to_string());
        }
        _ => {}
    } }
        }
        _ => {}
    } }
        { let mut guard = item.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = category.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Complex for loop conditions
    println!("{}", "\n=== Complex for loop conditions ===".to_string());

        // Multiple variables in for loop
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(10))));
    while (*i.lock().unwrap().as_ref().unwrap()) < (*j.lock().unwrap().as_ref().unwrap()) {
        print!("i={}, j={}, sum={}\n", (*i.lock().unwrap().as_ref().unwrap()), (*j.lock().unwrap().as_ref().unwrap()), (*i.lock().unwrap().as_ref().unwrap()) + (*j.lock().unwrap().as_ref().unwrap()));
        if (*i.lock().unwrap().as_ref().unwrap()) >= 3 {
        break
    }
        { let __tmp_0 = (*i.lock().unwrap().as_ref().unwrap()) + 1; let __tmp_1 = (*j.lock().unwrap().as_ref().unwrap()) - 1; *i.lock().unwrap() = Some(__tmp_0); *j.lock().unwrap() = Some(__tmp_1); };
    }

        // For loop with complex condition
    println!("{}", "\n=== For loop with complex condition ===".to_string());

    let (mut x, mut y) = (Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some(1))));
    while (*x.lock().unwrap().as_ref().unwrap()) * (*y.lock().unwrap().as_ref().unwrap()) < 100 && (*x.lock().unwrap().as_ref().unwrap()) < 10 {
        print!("x={}, y={}, product={}\n", (*x.lock().unwrap().as_ref().unwrap()), (*y.lock().unwrap().as_ref().unwrap()), (*x.lock().unwrap().as_ref().unwrap()) * (*y.lock().unwrap().as_ref().unwrap()));
        if (*x.lock().unwrap().as_ref().unwrap()) % 2 == 0 {
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 2); };
    } else {
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
    }
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Goto statements (rarely used, but valid Go)
    println!("{}", "\n=== Goto statements ===".to_string());

    let mut counter = Arc::new(Mutex::new(Some(0)));

    { let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    print!("Counter: {}\n", (*counter.lock().unwrap().as_ref().unwrap()));

    if (*counter.lock().unwrap().as_ref().unwrap()) < 3 {
        // TODO: goto not supported
    }

    println!("{}", "Done with goto".to_string());

        // Complex if-else chains
    println!("{}", "\n=== Complex if-else chains ===".to_string());

    let mut score = Arc::new(Mutex::new(Some(0)));
    while (*score.lock().unwrap().as_ref().unwrap()) <= 100 {
        let mut grade: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut message: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

        if (*score.lock().unwrap().as_ref().unwrap()) >= 90 {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
        if (*score.lock().unwrap().as_ref().unwrap()) >= 95 {
        { let new_val = "Excellent!".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "Great job!".to_string(); *message.lock().unwrap() = Some(new_val); };
    }
    } else if (*score.lock().unwrap().as_ref().unwrap()) >= 80 {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
        if (*score.lock().unwrap().as_ref().unwrap()) >= 85 {
        { let new_val = "Good work!".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "Not bad!".to_string(); *message.lock().unwrap() = Some(new_val); };
    }
    } else if (*score.lock().unwrap().as_ref().unwrap()) >= 70 {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Average".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_ref().unwrap()) >= 60 {
        { let new_val = "D".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Below average".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Needs improvement".to_string(); *message.lock().unwrap() = Some(new_val); };
    }

        print!("Score {}: Grade {} - {}\n", (*score.lock().unwrap().as_ref().unwrap()), (*grade.lock().unwrap().as_ref().unwrap()), (*message.lock().unwrap().as_ref().unwrap()));
        { let mut guard = score.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 25); };
    }

        // Range with complex break/continue logic
    println!("{}", "\n=== Range with complex break/continue ===".to_string());

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Processing numbers:".to_string());
    for (i, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().copied().enumerate() {
        if num % 2 == 0 {
        if num > 6 {
        print!("Stopping at even number {} (index {})\n", num, i);
        break
    }
        print!("Skipping even number {} (index {})\n", num, i);
        continue
    }
        if num == 7 {
        print!("Found lucky number {} at index {}\n", num, i);
        continue
    }
        print!("Processing odd number {} (index {})\n", num, i);
    }

        // Nested range loops
    println!("{}", "\n=== Nested range loops ===".to_string());

    let mut matrix = Arc::new(Mutex::new(Some(vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["d".to_string(), "e".to_string(), "f".to_string()], vec!["g".to_string(), "h".to_string(), "i".to_string()]])));

    for (rowIdx, row) in (*matrix.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        for (colIdx, cell) in row.iter().enumerate() {
        if cell == "e" {
        print!("Found center at [{}][{}]: {}\n", rowIdx, colIdx, cell);
        continue
    }
        if rowIdx == 2 && colIdx == 2 {
        print!("Last cell [{}][{}]: {}\n", rowIdx, colIdx, cell);
        break
    }
        print!("[{}][{}]: {} ", rowIdx, colIdx, cell);
    }
        println!();
    }

        // Select with complex channel operations
    println!("{}", "\n=== Select with complex channel operations ===".to_string());

    let mut ch1 = GoChannel::<i32>::new_buffered(2 as usize);
    let mut ch2 = GoChannel::<String>::new_buffered(2 as usize);
    let mut done = GoChannel::<bool>::new();

        // Fill channels
    ch1.send(1);
    ch1.send(2);
    ch2.send("hello".to_string());
    ch2.send("world".to_string());

    let ch1_closure_clone = ch1.clone(); let ch2_closure_clone = ch2.clone(); let done_closure_clone = done.clone(); let ch1_thread = ch1.clone(); let ch2_thread = ch2.clone(); let done_thread = done.clone(); std::thread::spawn(move || {
        let mut count = Arc::new(Mutex::new(Some(0)));;
        while true {
        loop {
        if let Some(val) = ch1_thread.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received int: {}\n", (*val.lock().unwrap().as_ref().unwrap()));
            { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            break;
        }
        if let Some(val) = ch2_thread.try_recv() {
            let mut val = Arc::new(Mutex::new(Some(val)));
            print!("Received string: {}\n", (*val.lock().unwrap().as_ref().unwrap()));
            { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            break;
        }
        if (*count.lock().unwrap().as_ref().unwrap()) >= 4 {
        done_thread.send(true);
        return;
    }
        break;
    }
    };;
    });

    done.recv().unwrap();
    println!("{}", "Channel processing complete".to_string());

        // Complex error handling flow
    println!("{}", "\n=== Complex error handling flow ===".to_string());

    let mut processData = Arc::new(Mutex::new(Some(Box::new(move |data: Arc<Mutex<Option<Vec<i32>>>>| -> Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>> {
        if (*data.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return Arc::new(Mutex::new(Some(Box::<dyn Error + Send + Sync>::from(format!("empty data")))));
    }
        for (i, val) in (*data.lock().unwrap().as_ref().unwrap()).iter().copied().enumerate() {
        if val < 0 {
        return Arc::new(Mutex::new(Some(Box::<dyn Error + Send + Sync>::from(format!("negative value at index {}: {}", i, val)))));
    }
        if val > 100 {
        return Arc::new(Mutex::new(Some(Box::<dyn Error + Send + Sync>::from(format!("value too large at index {}: {}", i, val)))));
    }
    }
        return Arc::new(Mutex::new(None));
    }) as Box<dyn Fn(Arc<Mutex<Option<Vec<i32>>>>) -> Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>> + Send + Sync>)));

    let mut testData = Arc::new(Mutex::new(Some(vec![vec![1, 2, 3], vec![], vec![1, -2, 3], vec![1, 200, 3], vec![10, 20, 30]])));

    for (i, data) in (*testData.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("Testing dataset {}: {}\n", i + 1, format_slice(&data));
        let mut err = (*processData.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(*data))));
    if (*err.lock().unwrap()).is_some() {
        print!("  Error: {}\n", format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        continue
    }
        print!("  Success: data is valid\n");
    }
}