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

struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i32>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i32) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
}

struct GoMutex {
    inner: std::sync::Arc<std::sync::Mutex<()>>,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new(std::sync::Mutex::new(())),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<()> {
        self.inner.lock().unwrap()
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        GoMutex {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
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

fn main() {
        // Worker pool pattern
    println!("{}", format!("{}", "=== Worker Pool Pattern ===".to_string()));

    let mut jobs = GoChannel::<i32>::new_buffered(100 as usize);
    let mut results = GoChannel::<i32>::new_buffered(100 as usize);

        // Start workers
    let mut wg = WaitGroup::new();
    let mut numWorkers = Arc::new(Mutex::new(Some(3)));

    let mut w = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*numWorkers.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        wg.add(1);
        let jobs_thread = jobs.clone(); let results_thread = results.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let __closure = move |id: Arc<Mutex<Option<i32>>>| {
            let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            for job in jobs_thread.clone() {
        print!("Worker {} processing job {}\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }, job);
        std::thread::sleep(std::time::Duration::from_millis(10));
        results_thread.send({ let __tmp_x = job; let __tmp_y = 2; __tmp_x * __tmp_y });
    };
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        };
        __closure(w.clone());
    });
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // Send jobs
    let mut numJobs = Arc::new(Mutex::new(Some(9)));
    let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*numJobs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        jobs.send((*j.lock().unwrap().as_ref().unwrap()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    jobs.close();

        // Wait for workers to finish
    let results_thread = results.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        wg_thread.wait();;
        results_thread.close();;;
    });

        // Collect results
    for result in results.clone() {
        print!("Result: {}\n", result);
    }

        // Producer-Consumer pattern
    println!("{}", format!("{}", "\n=== Producer-Consumer Pattern ===".to_string()));

    let mut buffer = GoChannel::<String>::new_buffered(5 as usize);
    let mut done = GoChannel::<bool>::new();

        // Producer
    let buffer_thread = buffer.clone(); std::thread::spawn(move || {
        let mut items = Arc::new(Mutex::new(Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string(), "elderberry".to_string()])));;
        { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        print!("Producing: {}\n", item);
        buffer_thread.send(item);
        std::thread::sleep(std::time::Duration::from_millis(50));
    } };
        buffer_thread.close();;;
    });

        // Consumer
    let buffer_thread = buffer.clone(); let done_thread = done.clone(); std::thread::spawn(move || {
        for item in buffer_thread.clone() {
        print!("Consuming: {}\n", item);
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
        done_thread.send(true);;;
    });

    done.recv().unwrap_or_default();

        // Fan-out/Fan-in pattern
    println!("{}", format!("{}", "\n=== Fan-out/Fan-in Pattern ===".to_string()));

    let mut input = GoChannel::<i32>::new();

        // Fan-out: distribute work to multiple goroutines
    let mut c1 = fan_out(input.clone());
    let mut c2 = fan_out(input.clone());
    let mut c3 = fan_out(input.clone());

        // Fan-in: combine results from multiple goroutines
    let mut output = fan_in(Arc::new(Mutex::new(Some(vec![c1, c2, c3]))));

        // Send input
    let input_thread = input.clone(); std::thread::spawn(move || {
        let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x <= __tmp_y } {
        input_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        input_thread.close();;;
    });

        // Collect output
    for result in output.clone() {
        print!("Fan-in result: {}\n", result);
    }

        // Pipeline pattern
    println!("{}", format!("{}", "\n=== Pipeline Pattern ===".to_string()));

        // Stage 1: Generate numbers
    let mut numbers = GoChannel::<i32>::new();
    let numbers_thread = numbers.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let numbers_defer_captured = numbers_thread.clone(); __defer_stack.push(Box::new(move || {
        numbers_defer_captured.close();
    }));;
        let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        numbers_thread.send((*i.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });

        // Stage 2: Square numbers
    let mut squares = GoChannel::<i32>::new();
    let numbers_thread = numbers.clone(); let squares_thread = squares.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let squares_defer_captured = squares_thread.clone(); __defer_stack.push(Box::new(move || {
        squares_defer_captured.close();
    }));;
        for n in numbers_thread.clone() {
        squares_thread.send({ let __tmp_x = n; let __tmp_y = n; __tmp_x * __tmp_y });
    };
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });

        // Stage 3: Add 10 to each
    let mut r#final = GoChannel::<i32>::new();
    let final_thread = final.clone(); let squares_thread = squares.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let final_defer_captured = final_thread.clone(); __defer_stack.push(Box::new(move || {
        final_defer_captured.close();
    }));;
        for s in squares_thread.clone() {
        final_thread.send({ let __tmp_x = s; let __tmp_y = 10; __tmp_x + __tmp_y });
    };
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });

        // Consume final results
    for result in r#final.clone() {
        print!("Pipeline result: {}\n", result);
    }

        // Mutex and shared state
    println!("{}", format!("{}", "\n=== Mutex and Shared State ===".to_string()));

    let mut counter: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut mutex = GoMutex::new();
    let mut wg2 = WaitGroup::new();

    let counter_closure_clone = counter.clone(); let mutex_closure_clone = mutex.clone(); let wg2_closure_clone = wg2.clone(); let mut increment = Arc::new(Mutex::new(Some(Box::new(move |id: Arc<Mutex<Option<i32>>>| {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let wg2_defer_captured = wg2_closure_clone.clone(); __defer_stack.push(Box::new(move || {
        wg2_defer_captured.done();
    }));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1000; __tmp_x < __tmp_y } {
        let __mutex_guard_source_2642 = mutex_closure_clone.clone(); let __mutex_guard_2642 = __mutex_guard_source_2642.lock();
        { let mut guard = counter_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        drop(__mutex_guard_2642);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        print!("Goroutine {} finished\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>)));

        // Start multiple goroutines
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        wg2.add(1);
        let i_thread = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); let increment_thread = Arc::new(Mutex::new(Some((*increment.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = increment_thread.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(i_thread.clone()) };
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    wg2.wait();
    print!("Final counter value: {}\n", { let __v = (*counter.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Channel-based synchronization
    println!("{}", format!("{}", "\n=== Channel-based Synchronization ===".to_string()));

    let mut wg3 = WaitGroup::new();
    let mut barrier = GoChannel::<bool>::new_buffered(3 as usize);

    let barrier_closure_clone = barrier.clone(); let wg3_closure_clone = wg3.clone(); let mut worker = Arc::new(Mutex::new(Some(Box::new(move |id: Arc<Mutex<Option<i32>>>| {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let wg3_defer_captured = wg3_closure_clone.clone(); __defer_stack.push(Box::new(move || {
        wg3_defer_captured.done();
    }));
        print!("Worker {}: Phase 1 complete\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
        barrier_closure_clone.send(true);
        barrier_closure_clone.recv().unwrap_or_default();
        barrier_closure_clone.recv().unwrap_or_default();
        barrier_closure_clone.recv().unwrap_or_default();
        print!("Worker {}: Phase 2 complete\n", { let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v });
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync>)));

        // Do some work
        // Signal completion of phase 1
        // Wait for all workers to complete phase 1
        // Do phase 2 work
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        wg3.add(1);
        let i_thread = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); let worker_thread = Arc::new(Mutex::new(Some((*worker.lock().unwrap().as_ref().unwrap()).clone()))); std::thread::spawn(move || {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> = { let mut __f_guard = worker_thread.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(i_thread.clone()) };
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    wg3.wait();

        // Timeout pattern
    println!("{}", format!("{}", "\n=== Timeout Pattern ===".to_string()));

    let mut slowOperation = Arc::new(Mutex::new(Some(Box::new(move || -> GoChannel<String> {
        let mut result = GoChannel::<String>::new();
        let result_thread = result.clone(); std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(200));;
        result_thread.send("Operation completed".to_string());;;
    });
        return result.clone();
    }) as Box<dyn FnMut() -> GoChannel<String> + Send + Sync>)));

    loop {
        if let Some(result) = { let __f_ptr: *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync> = { let mut __f_guard = slowOperation.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.try_recv() {
            let mut result = Arc::new(Mutex::new(Some(result)));
            print!("Success: {}\n", { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(100)).try_recv() {
            println!("{}", format!("{}", "Timeout: Operation took too long".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

        // Try again with longer timeout
    loop {
        if let Some(result) = { let __f_ptr: *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync> = { let mut __f_guard = slowOperation.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> GoChannel<String> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.try_recv() {
            let mut result = Arc::new(Mutex::new(Some(result)));
            print!("Success: {}\n", { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v });
            break;
        }
        if let Some(_) = go_channel_after(std::time::Duration::from_millis(300)).try_recv() {
            println!("{}", format!("{}", "Timeout: Operation took too long".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

/// Helper functions for fan-out/fan-in
pub fn fan_out(input: GoChannel<i32>) -> GoChannel<i32> {

    let mut output = GoChannel::<i32>::new();
    let input_thread = input.clone(); let output_thread = output.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let output_defer_captured = output_thread.clone(); __defer_stack.push(Box::new(move || {
        output_defer_captured.close();
    }));;
        for n in input_thread.clone() {
        output_thread.send({ let __tmp_x = n; let __tmp_y = n; __tmp_x * __tmp_y });
    };
        while let Some(f) = __defer_stack.pop() {
            f();
        };
    });
        // Square the number
    return output.clone();
}

pub fn fan_in(inputs: Arc<Mutex<Option<Vec<GoChannel<i32>>>>>) -> GoChannel<i32> {

    let mut output = GoChannel::<i32>::new();
    let mut wg = WaitGroup::new();

    { let __range_holder = inputs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for input in __range_values.iter() {
        wg.add(1);
        let output_thread = output.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();
        let __closure = move |ch: GoChannel<i32>| {
            let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            for n in (*ch.lock().unwrap().as_ref().unwrap()).clone() {
        output_thread.send(n);
    };
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        };
        __closure(Arc::new(Mutex::new(Some(input))));
    });
    } }

    let output_thread = output.clone(); let wg_thread = wg.clone(); std::thread::spawn(move || {
        wg_thread.wait();;
        output_thread.close();;;
    });

    return output.clone();
}