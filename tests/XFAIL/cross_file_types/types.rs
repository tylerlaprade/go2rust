use std::sync::{Arc, Mutex};

/// Person represents a person with name and age
#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

/// Address represents a physical address
#[derive(Debug)]
struct Address {
    street: Arc<Mutex<Option<String>>>,
    city: Arc<Mutex<Option<String>>>,
    zip: Arc<Mutex<Option<String>>>,
}

/// Employee combines Person and Address
#[derive(Debug)]
struct Employee {
    person: Arc<Mutex<Option<Person>>>,
    address: Arc<Mutex<Option<Address>>>,
    i_d: Arc<Mutex<Option<i32>>>,
}