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
        print!("Hello, I'm {} and I'm {} years old\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.age.lock().unwrap().as_mut().unwrap()));
    }

    pub fn have_birthday(&mut self) {
        { let mut guard = self.age.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("{} is now {} years old\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.age.lock().unwrap().as_mut().unwrap()));
    }
}

fn main() {
    let mut counter = Arc::new(Mutex::new(Some(Counter { value: Arc::new(Mutex::new(Some(0))) })));
    println!("{} {}", "Initial value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_mut().unwrap()));

    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "After increment:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_mut().unwrap()));

    (*counter.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(5))));
    println!("{} {}", "After adding 5:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_mut().unwrap()));

    let mut doubled = (*counter.lock().unwrap().as_mut().unwrap()).double();
    println!("{} {}", "After doubling:".to_string(), (*doubled.lock().unwrap().as_mut().unwrap()));

    let mut person = Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(25))) })));
    (*person.lock().unwrap().as_mut().unwrap()).greet();
    (*person.lock().unwrap().as_mut().unwrap()).have_birthday();
    (*person.lock().unwrap().as_mut().unwrap()).greet();
}