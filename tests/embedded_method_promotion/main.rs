use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


thread_local! {
    static GO_EMBEDDED_OWNER_REGISTRY: RefCell<std::collections::HashMap<usize, Box<dyn Any>>> = RefCell::new(std::collections::HashMap::new());
}

fn go_register_embedded_owner<T: 'static>(embedded_key: usize, owner: Rc<RefCell<Option<T>>>) {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(embedded_key, Box::new(owner));
    });
}

fn go_lookup_embedded_owner<T: 'static>(embedded_key: usize, target: &str) -> Rc<RefCell<Option<T>>> {
    GO_EMBEDDED_OWNER_REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
        owner
            .downcast_ref::<Rc<RefCell<Option<T>>>>()
            .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
            .clone()
    })
}

/// Base types with methods
#[derive(Debug, Clone)]
pub struct Logger {
    pub prefix: Rc<RefCell<Option<String>>>,
}

impl Logger {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.prefix.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            prefix: __go_clone_0_0,
        }
    }
}


impl Default for Logger {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            prefix: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.prefix.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Counter {
    pub count: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            count: __go_clone_0_0,
        }
    }
}


impl Default for Counter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            count: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.count.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// Type that embeds both Logger and Counter
#[derive(Debug, Clone)]
pub struct Service {
    pub logger: Rc<RefCell<Option<Logger>>>,
    pub counter: Rc<RefCell<Option<Counter>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl Service {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.logger.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.counter.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            logger: __go_clone_0_0,
            counter: __go_clone_1_0,
            name: __go_clone_2_0,
        }
    }
}


impl Default for Service {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Logger::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Counter::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            logger: __go_default_0_0,
            counter: __go_default_1_0,
            name: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.logger.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.counter.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


/// Type with multiple levels of embedding
#[derive(Debug, Clone)]
pub struct Base {
    pub id: Rc<RefCell<Option<i32>>>,
}

impl Base {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            id: __go_clone_0_0,
        }
    }
}


impl Default for Base {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            id: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Middle {
    pub base: Rc<RefCell<Option<Base>>>,
    pub data: Rc<RefCell<Option<String>>>,
}

impl Middle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.data.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            data: __go_clone_1_0,
        }
    }
}


impl Default for Middle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Base::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            base: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Middle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.data.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Top {
    pub middle: Rc<RefCell<Option<Middle>>>,
    pub extra: Rc<RefCell<Option<String>>>,
}

impl Top {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.middle.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.extra.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            middle: __go_clone_0_0,
            extra: __go_clone_1_0,
        }
    }
}


impl Default for Top {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Middle::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            middle: __go_default_0_0,
            extra: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Top {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.middle.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.extra.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Logger {
    pub fn log(&self, msg: Rc<RefCell<Option<String>>>) {
        print!("[{}] {}\n", (*self.prefix.borrow().as_ref().unwrap()), { let __v = (*msg.borrow().as_ref().unwrap()).clone(); __v });
    }

    pub fn set_prefix(&mut self, prefix: Rc<RefCell<Option<String>>>) {
        { let new_val = prefix.borrow().as_ref().unwrap().clone(); *self.prefix.borrow_mut() = Some(new_val); };
    }
}

impl Counter {
    pub fn value(&self) -> i32 {
        return (*self.count.borrow().as_ref().unwrap());
    }

    pub fn increment(&mut self) {
        { let __target = self.count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let __target = self.count.clone(); let __rhs = (*n.borrow().as_ref().unwrap()); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

impl Service {
    /// Service's own method
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    /// Method that shadows embedded method
    pub fn value(&self) -> i32 {
                // This should shadow Counter.Value()
        (*self.counter.borrow().as_ref().unwrap()).value() * 10
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
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
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
    pub fn get_i_d(&self) -> i32 {
        return (*self.id.borrow().as_ref().unwrap());
    }

    pub fn set_i_d(&mut self, id: Rc<RefCell<Option<i32>>>) {
        { let new_val = id.borrow().as_ref().unwrap().clone(); *self.id.borrow_mut() = Some(new_val); };
    }
}

impl Middle {
    pub fn get_data(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.data.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    pub fn get_i_d(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
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
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.extra.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    pub fn get_data(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get_data()
    }

    pub fn get_i_d(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.middle.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
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
    println!("{}", format!("{}", "=== Basic method promotion ===".to_string()));
    let mut svc = Rc::new(RefCell::new(Some(Service { logger: Rc::new(RefCell::new(Some(Logger { prefix: Rc::new(RefCell::new(Some("SVC".to_string()))), ..Default::default() }))), counter: Rc::new(RefCell::new(Some(Counter { count: Rc::new(RefCell::new(Some(0))), ..Default::default() }))), name: Rc::new(RefCell::new(Some("MyService".to_string()))), ..Default::default() })));

        // Call promoted methods from Logger
    (*svc.borrow().as_ref().unwrap()).log(Rc::new(RefCell::new(Some("Service started".to_string()))));
    (*svc.borrow_mut().as_mut().unwrap()).set_prefix(Rc::new(RefCell::new(Some("SERVICE".to_string()))));
    (*svc.borrow().as_ref().unwrap()).log(Rc::new(RefCell::new(Some("Prefix changed".to_string()))));

        // Call promoted methods from Counter
    (*svc.borrow_mut().as_mut().unwrap()).increment();
    (*svc.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(5))));
    print!("Counter value (via promoted method): {}\n", (*(*svc.borrow().as_ref().unwrap()).counter.borrow().as_ref().unwrap()).value());

        // Call Service's own methods
    print!("Service name: {}\n", (*(*svc.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap()));
    print!("Shadowed Value method: {}\n", (*svc.borrow().as_ref().unwrap()).value());

        // Test method promotion with pointers
    println!("{}", format!("{}", "\n=== Method promotion with pointers ===".to_string()));
    let mut svcPtr = {
        let __owner = Rc::new(RefCell::new(Some(Service { logger: Rc::new(RefCell::new(Some(Logger { prefix: Rc::new(RefCell::new(Some("PTR".to_string()))), ..Default::default() }))), counter: Rc::new(RefCell::new(Some(Counter { count: Rc::new(RefCell::new(Some(10))), ..Default::default() }))), name: Rc::new(RefCell::new(Some("PointerService".to_string()))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.borrow(); __owner_guard.as_ref().unwrap().logger.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.borrow(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };

    (*svcPtr.borrow().as_ref().unwrap()).log(Rc::new(RefCell::new(Some("Pointer service".to_string()))));
    (*svcPtr.borrow_mut().as_mut().unwrap()).increment();
    print!("Pointer service counter: {}\n", (*(*svcPtr.borrow().as_ref().unwrap()).counter.borrow().as_ref().unwrap()).value());

        // Test multi-level embedding
    println!("{}", format!("{}", "\n=== Multi-level embedding ===".to_string()));
    let mut top = Rc::new(RefCell::new(Some(Top { middle: Rc::new(RefCell::new(Some(Middle { base: Rc::new(RefCell::new(Some(Base { id: Rc::new(RefCell::new(Some(100))), ..Default::default() }))), data: Rc::new(RefCell::new(Some("middle data".to_string()))), ..Default::default() }))), extra: Rc::new(RefCell::new(Some("extra data".to_string()))), ..Default::default() })));

        // Methods promoted from Base through Middle
    print!("ID (promoted from Base): {}\n", (*top.borrow().as_ref().unwrap()).get_i_d());
    (*top.borrow_mut().as_mut().unwrap()).set_i_d(Rc::new(RefCell::new(Some(200))));
    print!("ID after SetID: {}\n", (*top.borrow().as_ref().unwrap()).get_i_d());

        // Methods promoted from Middle
    print!("Data (promoted from Middle): {}\n", (*(*top.borrow().as_ref().unwrap()).get_data().borrow().as_ref().unwrap()));

        // Top's own method
    print!("Extra: {}\n", (*(*top.borrow().as_ref().unwrap()).get_extra().borrow().as_ref().unwrap()));

        // Test with embedded pointer types would go here
        // but local type definitions with methods aren't supported in functions
    println!("{}", format!("{}", "\n=== End of method promotion tests ===".to_string()));
}