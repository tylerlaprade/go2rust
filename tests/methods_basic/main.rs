use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Counter {
    value: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

impl Counter {
    /// Method with value receiver
    pub fn get_value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }

    /// Method with pointer receiver
    pub fn increment(&mut self) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: Arc<Mutex<Option<i32>>>) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*n.lock().unwrap().as_mut().unwrap())); };
    }

    /// Method with return value
    pub fn double(&mut self) -> Arc<Mutex<Option<i32>>> {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * 2); };
        return self.value.clone();
    }
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {} and I'm {} years old\n", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.age.lock().unwrap().as_ref().unwrap()));
    }

    pub fn have_birthday(&mut self) {
        { let mut guard = self.age.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("{} is now {} years old\n", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.age.lock().unwrap().as_ref().unwrap()));
    }
}

fn main() {
        // Counter methods
    let mut counter = Arc::new(Mutex::new(Some(Counter { value: Arc::new(Mutex::new(Some(0))) })));
    println!("{} {}", "Initial value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_ref().unwrap()));

    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "After increment:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_ref().unwrap()));

    (*counter.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(5))));
    println!("{} {}", "After adding 5:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_ref().unwrap()));

    let mut doubled = (*counter.lock().unwrap().as_mut().unwrap()).double();
    println!("{} {}", "After doubling:".to_string(), (*doubled.lock().unwrap().as_mut().unwrap()));

        // Person methods
    let mut person = Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(25))) })));
    (*person.lock().unwrap().as_mut().unwrap()).greet();
    (*person.lock().unwrap().as_mut().unwrap()).have_birthday();
    (*person.lock().unwrap().as_mut().unwrap()).greet();
}