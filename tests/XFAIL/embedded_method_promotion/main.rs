use std::sync::{Arc, Mutex};

/// Base types with methods
#[derive(Debug)]
struct Logger {
    prefix: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Counter {
    count: Arc<Mutex<Option<i32>>>,
}

/// Type that embeds both Logger and Counter
#[derive(Debug)]
struct Service {
    logger: Arc<Mutex<Option<Logger>>>,
    counter: Arc<Mutex<Option<Counter>>>,
    name: Arc<Mutex<Option<String>>>,
}

/// Type with multiple levels of embedding
#[derive(Debug)]
struct Base {
    id: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Middle {
    base: Arc<Mutex<Option<Base>>>,
    data: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Top {
    middle: Arc<Mutex<Option<Middle>>>,
    extra: Arc<Mutex<Option<String>>>,
}

impl Logger {
    pub fn log(&self, msg: Arc<Mutex<Option<String>>>) {
        print!("[{}] {}\n", (*self.prefix.lock().unwrap().as_ref().unwrap()), (*msg.lock().unwrap().as_mut().unwrap()));
    }

    pub fn set_prefix(&mut self, prefix: Arc<Mutex<Option<String>>>) {
        { let new_val = (*prefix.lock().unwrap().as_mut().unwrap()); *self.prefix.lock().unwrap() = Some(new_val); };
    }
}

impl Counter {
    pub fn value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.count.clone();
    }

    pub fn increment(&mut self) {
        { let mut guard = self.count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Arc<Mutex<Option<i32>>>) {
        { let mut guard = self.count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*n.lock().unwrap().as_mut().unwrap())); };
    }
}

impl Service {
    /// Service's own method
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    /// Method that shadows embedded method
    pub fn value(&self) -> Arc<Mutex<Option<i32>>> {
                // This should shadow Counter.Value()
        return Arc::new(Mutex::new(Some((*(*(*self.logger.lock().unwrap().as_ref().unwrap()).counter.clone().lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()) * 10)));
    }

    pub fn add(&mut self, n: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.counter.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add(n)
    }

    pub fn increment(&mut self) {
        // Forward to embedded type's method
        let embedded = self.counter.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.increment()
    }

    pub fn log(&self, msg: Arc<Mutex<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.logger.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.log(msg)
    }

    pub fn set_prefix(&mut self, prefix: Arc<Mutex<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.logger.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_prefix(prefix)
    }
}

impl Base {
    pub fn get_i_d(&self) -> Arc<Mutex<Option<i32>>> {
        return self.id.clone();
    }

    pub fn set_i_d(&mut self, id: Arc<Mutex<Option<i32>>>) {
        { let new_val = (*id.lock().unwrap().as_mut().unwrap()); *self.id.lock().unwrap() = Some(new_val); };
    }
}

impl Middle {
    pub fn get_data(&self) -> Arc<Mutex<Option<String>>> {
        return self.data.clone();
    }

    pub fn get_i_d(&self) -> Arc<Mutex<Option<i32>>> {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_i_d()
    }

    pub fn set_i_d(&mut self, id: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_i_d(id)
    }
}

impl Top {
    pub fn get_extra(&self) -> Arc<Mutex<Option<String>>> {
        return self.extra.clone();
    }

    pub fn get_data(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_data()
    }

    pub fn get_i_d(&self) -> Arc<Mutex<Option<i32>>> {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_i_d()
    }

    pub fn set_i_d(&mut self, id: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_i_d(id)
    }
}

fn main() {
        // Test basic method promotion
    println!("{}", "=== Basic method promotion ===".to_string());
    let mut svc = Service { logger: Arc::new(Mutex::new(Some(Logger { prefix: Arc::new(Mutex::new(Some("SVC".to_string()))) }))), counter: Arc::new(Mutex::new(Some(Counter { count: Arc::new(Mutex::new(Some(0))) }))), name: Arc::new(Mutex::new(Some("MyService".to_string()))) };

        // Call promoted methods from Logger
    svc.log(Arc::new(Mutex::new(Some("Service started".to_string()))));
    svc.set_prefix(Arc::new(Mutex::new(Some("SERVICE".to_string()))));
    svc.log(Arc::new(Mutex::new(Some("Prefix changed".to_string()))));

        // Call promoted methods from Counter
    svc.increment();
    svc.add(Arc::new(Mutex::new(Some(5))));
    print!("Counter value (via promoted method): {}\n", (*(*svc.logger.lock().unwrap().as_ref().unwrap().counter.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));

        // Call Service's own methods
    print!("Service name: {}\n", (*svc.name().lock().unwrap().as_ref().unwrap()));
    print!("Shadowed Value method: {}\n", (*svc.value().lock().unwrap().as_ref().unwrap()));

        // Test method promotion with pointers
    println!("{}", "\n=== Method promotion with pointers ===".to_string());
    let mut svcPtr = Arc::new(Mutex::new(Some(Service { logger: Arc::new(Mutex::new(Some(Logger { prefix: Arc::new(Mutex::new(Some("PTR".to_string()))) }))), counter: Arc::new(Mutex::new(Some(Counter { count: Arc::new(Mutex::new(Some(10))) }))), name: Arc::new(Mutex::new(Some("PointerService".to_string()))) })));

    (*svcPtr.lock().unwrap().as_mut().unwrap()).log(Arc::new(Mutex::new(Some("Pointer service".to_string()))));
    (*svcPtr.lock().unwrap().as_mut().unwrap()).increment();
    print!("Pointer service counter: {}\n", (*(*svcPtr.logger.lock().unwrap().as_ref().unwrap().counter.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));

        // Test multi-level embedding
    println!("{}", "\n=== Multi-level embedding ===".to_string());
    let mut top = Top { middle: Arc::new(Mutex::new(Some(Middle { base: Arc::new(Mutex::new(Some(Base { id: Arc::new(Mutex::new(Some(100))) }))), data: Arc::new(Mutex::new(Some("middle data".to_string()))) }))), extra: Arc::new(Mutex::new(Some("extra data".to_string()))) };

        // Methods promoted from Base through Middle
    print!("ID (promoted from Base): {}\n", (*top.get_i_d().lock().unwrap().as_ref().unwrap()));
    top.set_i_d(Arc::new(Mutex::new(Some(200))));
    print!("ID after SetID: {}\n", (*top.get_i_d().lock().unwrap().as_ref().unwrap()));

        // Methods promoted from Middle
    print!("Data (promoted from Middle): {}\n", (*top.get_data().lock().unwrap().as_ref().unwrap()));

        // Top's own method
    print!("Extra: {}\n", (*top.get_extra().lock().unwrap().as_ref().unwrap()));

        // Test with embedded pointer types would go here
        // but local type definitions with methods aren't supported in functions
    println!("{}", "\n=== End of method promotion tests ===".to_string());
}