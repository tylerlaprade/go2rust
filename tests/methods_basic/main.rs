fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
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
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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