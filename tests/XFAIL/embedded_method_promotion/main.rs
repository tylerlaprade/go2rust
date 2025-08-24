use std::cell::{RefCell};
use std::rc::{Rc};

/// Base types with methods
#[derive(Debug, Clone, Default)]
struct Logger {
    prefix: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Counter {
    count: Rc<RefCell<Option<i32>>>,
}

/// Type that embeds both Logger and Counter
#[derive(Debug, Clone, Default)]
struct Service {
    logger: Rc<RefCell<Option<Logger>>>,
    counter: Rc<RefCell<Option<Counter>>>,
    name: Rc<RefCell<Option<String>>>,
}

/// Type with multiple levels of embedding
#[derive(Debug, Clone, Default)]
struct Base {
    id: Rc<RefCell<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct Middle {
    base: Rc<RefCell<Option<Base>>>,
    data: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Top {
    middle: Rc<RefCell<Option<Middle>>>,
    extra: Rc<RefCell<Option<String>>>,
}

impl Logger {
    pub fn log(&self, msg: Rc<RefCell<Option<String>>>) {
        print!("[{}] {}\n", (*self.prefix.borrow().as_ref().unwrap()), (*msg.borrow_mut().as_mut().unwrap()));
    }

    pub fn set_prefix(&mut self, prefix: Rc<RefCell<Option<String>>>) {
        { let new_val = (*prefix.borrow_mut().as_mut().unwrap()); *self.prefix.borrow_mut() = Some(new_val); };
    }
}

impl Counter {
    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.count.clone();
    }

    pub fn increment(&mut self) {
        { let mut guard = self.count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let mut guard = self.count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*n.borrow_mut().as_mut().unwrap())); };
    }
}

impl Service {
    /// Service's own method
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }

    /// Method that shadows embedded method
    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
                // This should shadow Counter.Value()
        return Rc::new(RefCell::new(Some((*(*(*self.logger.borrow().as_ref().unwrap()).counter.clone().borrow().as_mut().unwrap()).value().borrow().as_ref().unwrap()) * 10)));
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.counter.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.add(n)
    }

    pub fn increment(&mut self) {
        // Forward to embedded type's method
        let embedded = self.counter.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.increment()
    }

    pub fn log(&self, msg: Rc<RefCell<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.logger.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.log(msg)
    }

    pub fn set_prefix(&mut self, prefix: Rc<RefCell<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.logger.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_prefix(prefix)
    }
}

impl Base {
    pub fn get_i_d(&self) -> Rc<RefCell<Option<i32>>> {
        return self.id.clone();
    }

    pub fn set_i_d(&mut self, id: Rc<RefCell<Option<i32>>>) {
        { let new_val = (*id.borrow_mut().as_mut().unwrap()); *self.id.borrow_mut() = Some(new_val); };
    }
}

impl Middle {
    pub fn get_data(&self) -> Rc<RefCell<Option<String>>> {
        return self.data.clone();
    }

    pub fn get_i_d(&self) -> Rc<RefCell<Option<i32>>> {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_i_d()
    }

    pub fn set_i_d(&mut self, id: Rc<RefCell<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_i_d(id)
    }
}

impl Top {
    pub fn get_extra(&self) -> Rc<RefCell<Option<String>>> {
        return self.extra.clone();
    }

    pub fn get_data(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_data()
    }

    pub fn get_i_d(&self) -> Rc<RefCell<Option<i32>>> {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_i_d()
    }

    pub fn set_i_d(&mut self, id: Rc<RefCell<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_i_d(id)
    }
}

fn main() {
        // Test basic method promotion
    println!("{}", "=== Basic method promotion ===".to_string());
    let mut svc = Rc::new(RefCell::new(Some(Service { logger: Rc::new(RefCell::new(Some(Logger { prefix: Rc::new(RefCell::new(Some("SVC".to_string()))) }))), counter: Rc::new(RefCell::new(Some(Counter { count: Rc::new(RefCell::new(Some(0))) }))), name: Rc::new(RefCell::new(Some("MyService".to_string()))) })));

        // Call promoted methods from Logger
    (*svc.borrow_mut().as_mut().unwrap()).log(Rc::new(RefCell::new(Some("Service started".to_string()))));
    (*svc.borrow_mut().as_mut().unwrap()).set_prefix(Rc::new(RefCell::new(Some("SERVICE".to_string()))));
    (*svc.borrow_mut().as_mut().unwrap()).log(Rc::new(RefCell::new(Some("Prefix changed".to_string()))));

        // Call promoted methods from Counter
    (*svc.borrow_mut().as_mut().unwrap()).increment();
    (*svc.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(5))));
    print!("Counter value (via promoted method): {}\n", (*(*(*(*(*svc.borrow().as_ref().unwrap()).logger.borrow().as_ref().unwrap()).counter.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).value().borrow().as_ref().unwrap()));

        // Call Service's own methods
    print!("Service name: {}\n", (*(*svc.borrow_mut().as_mut().unwrap()).name().borrow().as_ref().unwrap()));
    print!("Shadowed Value method: {}\n", (*(*svc.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));

        // Test method promotion with pointers
    println!("{}", "\n=== Method promotion with pointers ===".to_string());
    let mut svcPtr = Rc::new(RefCell::new(Some(Service { logger: Rc::new(RefCell::new(Some(Logger { prefix: Rc::new(RefCell::new(Some("PTR".to_string()))) }))), counter: Rc::new(RefCell::new(Some(Counter { count: Rc::new(RefCell::new(Some(10))) }))), name: Rc::new(RefCell::new(Some("PointerService".to_string()))) })));

    (*svcPtr.borrow_mut().as_mut().unwrap()).log(Rc::new(RefCell::new(Some("Pointer service".to_string()))));
    (*svcPtr.borrow_mut().as_mut().unwrap()).increment();
    print!("Pointer service counter: {}\n", (*(*(*(*(*svcPtr.borrow().as_ref().unwrap()).logger.borrow().as_ref().unwrap()).counter.borrow().as_ref().unwrap()).borrow().as_mut().unwrap()).value().borrow().as_ref().unwrap()));

        // Test multi-level embedding
    println!("{}", "\n=== Multi-level embedding ===".to_string());
    let mut top = Rc::new(RefCell::new(Some(Top { middle: Rc::new(RefCell::new(Some(Middle { base: Rc::new(RefCell::new(Some(Base { id: Rc::new(RefCell::new(Some(100))) }))), data: Rc::new(RefCell::new(Some("middle data".to_string()))) }))), extra: Rc::new(RefCell::new(Some("extra data".to_string()))) })));

        // Methods promoted from Base through Middle
    print!("ID (promoted from Base): {}\n", (*(*top.borrow_mut().as_mut().unwrap()).get_i_d().borrow().as_ref().unwrap()));
    (*top.borrow_mut().as_mut().unwrap()).set_i_d(Rc::new(RefCell::new(Some(200))));
    print!("ID after SetID: {}\n", (*(*top.borrow_mut().as_mut().unwrap()).get_i_d().borrow().as_ref().unwrap()));

        // Methods promoted from Middle
    print!("Data (promoted from Middle): {}\n", (*(*top.borrow_mut().as_mut().unwrap()).get_data().borrow().as_ref().unwrap()));

        // Top's own method
    print!("Extra: {}\n", (*(*top.borrow_mut().as_mut().unwrap()).get_extra().borrow().as_ref().unwrap()));

        // Test with embedded pointer types would go here
        // but local type definitions with methods aren't supported in functions
    println!("{}", "\n=== End of method promotion tests ===".to_string());
}