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
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.value)));
    }

    pub fn increment(&mut self) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn add(&mut self, n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
        self.value += (*n.lock().unwrap().as_ref().unwrap());
    }

    pub fn double(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        self.value = 2;
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.value)));
    }
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {} and I'm {} years old\n", self.name, self.age);
    }

    pub fn have_birthday(&mut self) {
        { let mut guard = self.age.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("{} is now {} years old\n", self.name, self.age);
    }
}

fn main() {
    let mut counter = Counter { value: 0 }.clone();
    println!("{} {}", "Initial value:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).get_value());
    (*counter.lock().unwrap().as_ref().unwrap()).increment();
    println!("{} {}", "After increment:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).get_value());
    (*counter.lock().unwrap().as_ref().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    println!("{} {}", "After adding 5:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).get_value());
    let mut doubled = (*counter.lock().unwrap().as_ref().unwrap()).double();
    println!("{} {}", "After doubling:".to_string(), (*doubled.lock().unwrap().as_ref().unwrap()));
    let mut person = Person { name: "Alice".to_string(), age: 25 }.clone();
    (*person.lock().unwrap().as_ref().unwrap()).greet();
    (*person.lock().unwrap().as_ref().unwrap()).have_birthday();
    (*person.lock().unwrap().as_ref().unwrap()).greet();
}