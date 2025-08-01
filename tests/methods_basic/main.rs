#[derive(Debug)]
struct Counter {
    value: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

impl Counter {
    pub fn get_value(&self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return self.value.clone();
    }

    pub fn increment(&mut self) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (*n.lock().unwrap().as_mut().unwrap())); };
    }

    pub fn double(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * 2); };
        return self.value.clone();
    }
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {} and I'm {} years old\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.age.lock().unwrap().as_mut().unwrap()));
    }

    pub fn have_birthday(&mut self) {
        { let mut guard = self.age.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("{} is now {} years old\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.age.lock().unwrap().as_mut().unwrap()));
    }
}

fn main() {
    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(Counter { value: std::sync::Arc::new(std::sync::Mutex::new(Some(0))) })));
    println!("{} {}", "Initial value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_mut().unwrap()));
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "After increment:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_mut().unwrap()));
    (*counter.lock().unwrap().as_mut().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    println!("{} {}", "After adding 5:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_mut().unwrap()));
    let mut doubled = (*counter.lock().unwrap().as_mut().unwrap()).double();
    println!("{} {}", "After doubling:".to_string(), (*doubled.lock().unwrap().as_mut().unwrap()));
    let mut person = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(25))) })));
    (*person.lock().unwrap().as_mut().unwrap()).greet();
    (*person.lock().unwrap().as_mut().unwrap()).have_birthday();
    (*person.lock().unwrap().as_mut().unwrap()).greet();
}