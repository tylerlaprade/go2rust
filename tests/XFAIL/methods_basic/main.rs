#[derive(Debug)]
struct Counter {
    value: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

pub fn get_value() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()).value)));
}

pub fn increment() {
    { let mut guard = (*c.lock().unwrap().as_ref().unwrap()).value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

pub fn add(n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    (*c.lock().unwrap().as_ref().unwrap()).value += (*n.lock().unwrap().as_ref().unwrap());
}

pub fn double() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    (*c.lock().unwrap().as_ref().unwrap()).value = 2;
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()).value)));
}

#[derive(Debug)]
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

pub fn greet() {
    print!("Hello, I'm {} and I'm {} years old\n", (*p.lock().unwrap().as_ref().unwrap()).name, (*p.lock().unwrap().as_ref().unwrap()).age);
}

pub fn have_birthday() {
    { let mut guard = (*p.lock().unwrap().as_ref().unwrap()).age.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    print!("{} is now {} years old\n", (*p.lock().unwrap().as_ref().unwrap()).name, (*p.lock().unwrap().as_ref().unwrap()).age);
}

fn main() {
    let mut counter = Counter { value: 0 }.clone();
    println!("{} {}", "Initial value:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).get_value());
    (*counter.lock().unwrap().as_ref().unwrap()).increment();
    println!("{} {}", "After increment:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).get_value());
    (*counter.lock().unwrap().as_ref().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    println!("{} {}", "After adding 5:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).get_value());
    let mut doubled = std::sync::Arc::new(std::sync::Mutex::new(Some((*counter.lock().unwrap().as_ref().unwrap()).double())));
    println!("{} {}", "After doubling:".to_string(), (*doubled.lock().unwrap().as_ref().unwrap()));
    let mut person = Person { name: "Alice".to_string(), age: 25 }.clone();
    (*person.lock().unwrap().as_ref().unwrap()).greet();
    (*person.lock().unwrap().as_ref().unwrap()).have_birthday();
    (*person.lock().unwrap().as_ref().unwrap()).greet();
}