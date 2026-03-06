use std::cell::{RefCell};
use std::rc::{Rc};


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

pub fn worker(id: Rc<RefCell<Option<i32>>>, wg: WaitGroup) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        wg.done();
    }));
    print!("Worker {} starting\n", (*id.borrow().as_ref().unwrap()));
    print!("Worker {} done\n", (*id.borrow().as_ref().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    let mut wg = WaitGroup::new();
    let mut i = Rc::new(RefCell::new(Some(1)));
    while (*i.borrow().as_ref().unwrap()) <= 3 {
        wg.add(1);
        worker(i.clone(), wg.clone());
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg.wait();
    println!("{}", "All workers done".to_string());
}