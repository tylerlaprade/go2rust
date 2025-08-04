use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Address {
    street: Arc<Mutex<Option<String>>>,
    city: Arc<Mutex<Option<String>>>,
    state: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Employee {
    arc<_mutex<_option<_person>>>: Arc<Mutex<Option<Person>>>,
    arc<_mutex<_option<_address>>>: Arc<Mutex<Option<Address>>>,
    i_d: Arc<Mutex<Option<i32>>>,
    salary: Arc<Mutex<Option<f64>>>,
}

fn main() {
    let mut p1 = Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) };
    println!("{} {}", "Person 1:".to_string(), (*p1.lock().unwrap().as_mut().unwrap()));

    let mut p2 = Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))) };
    println!("{} {}", "Person 2:".to_string(), (*p2.lock().unwrap().as_mut().unwrap()));

    { let new_val = 26; *(*p2.lock().unwrap().as_mut().unwrap()).age.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Updated Person 2:".to_string(), (*p2.lock().unwrap().as_mut().unwrap()));

    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Charlie".to_string()))), age: Arc::new(Mutex::new(Some(35))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Anytown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(1001))), salary: Arc::new(Mutex::new(Some(75000.0))) };

    println!("{} {}", "Employee:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Employee name:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()).name);
    println!("{} {}", "Employee city:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()).city);
}