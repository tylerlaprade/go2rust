#[derive(Debug)]
struct Counter {
    value: i32,
}

pub fn get_value() -> i32 {

    return c.value;
}

pub fn increment() {
    c.value += 1;
}

pub fn add(n: i32) {
    c.value += n;
}

pub fn double() -> i32 {

    c.value = 2;
    return c.value;
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

pub fn greet() {
    print!("Hello, I'm {} and I'm {} years old\n", p.name, p.age);
}

pub fn have_birthday() {
    p.age += 1;
    print!("{} is now {} years old\n", p.name, p.age);
}

fn main() {
    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(Counter { value: 0 })));
    println!("{} {}", "Initial value:".to_string(), counter.get_value());
    counter.increment();
    println!("{} {}", "After increment:".to_string(), counter.get_value());
    counter.add(5);
    println!("{} {}", "After adding 5:".to_string(), counter.get_value());
    let mut doubled = counter.double();
    println!("{} {}", "After doubling:".to_string(), doubled);
    let mut person = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Alice".to_string(), age: 25 })));
    person.greet();
    person.have_birthday();
    person.greet();
}