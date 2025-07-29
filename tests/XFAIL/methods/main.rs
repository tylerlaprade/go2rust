#[derive(Debug)]
struct Counter {
    value: i32,
}

pub fn increment() {
    c.value += 1;
}

pub fn value() -> i32 {

    return c.value;
}

pub fn new_counter() -> std::sync::Arc<std::sync::Mutex<Option<Counter>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(Counter { value: 0 })));
}

fn main() {
    let mut counter = new_counter();
    counter.increment();
    counter.increment();
    println!("{} {}", "Counter value:".to_string(), counter.value());
}