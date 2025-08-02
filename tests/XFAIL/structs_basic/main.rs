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
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Address {
    street: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    city: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    state: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Employee {
    std::sync::_arc<std::sync::_mutex<_option<_person>>>: std::sync::Arc<std::sync::Mutex<Option<Person>>>,
    std::sync::_arc<std::sync::_mutex<_option<_address>>>: std::sync::Arc<std::sync::Mutex<Option<Address>>>,
    i_d: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    salary: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

fn main() {
    let mut p1 = Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(30))) };
    println!("{} {}", "Person 1:".to_string(), (*p1.lock().unwrap().as_mut().unwrap()));

    let mut p2 = Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Bob".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(25))) };
    println!("{} {}", "Person 2:".to_string(), (*p2.lock().unwrap().as_mut().unwrap()));

    { let new_val = 26; *(*p2.lock().unwrap().as_mut().unwrap()).age.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Updated Person 2:".to_string(), (*p2.lock().unwrap().as_mut().unwrap()));

    let mut emp = Employee { person: std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Charlie".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(35))) }))), address: std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("123 Main St".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Anytown".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))) }))), i_d: std::sync::Arc::new(std::sync::Mutex::new(Some(1001))), salary: std::sync::Arc::new(std::sync::Mutex::new(Some(75000.0))) };

    println!("{} {}", "Employee:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Employee name:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()).name);
    println!("{} {}", "Employee city:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()).city);
}