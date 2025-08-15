use std::cmp::Ord;
use std::collections::HashMap;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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

fn main() {
        // Create and initialize map
    let mut ages = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<i32>>>>::new())));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Alice".to_string(), Arc::new(Mutex::new(Some(25))));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Bob".to_string(), Arc::new(Mutex::new(Some(30))));
    (*ages.lock().unwrap().as_mut().unwrap()).insert("Charlie".to_string(), Arc::new(Mutex::new(Some(35))));

    println!("{} {}", "Ages map:".to_string(), format_map(&ages));

        // Map literal
    let mut colors = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<String>>>>::from([("red".to_string(), Arc::new(Mutex::new(Some("#FF0000".to_string())))), ("green".to_string(), Arc::new(Mutex::new(Some("#00FF00".to_string())))), ("blue".to_string(), Arc::new(Mutex::new(Some("#0000FF".to_string()))))]))));

    println!("{} {}", "Colors map:".to_string(), format_map(&colors));

        // Check if key exists
    let (mut age, mut exists) = match (*ages.lock().unwrap().as_ref().unwrap()).get(&"Alice".to_string()) { Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false)))) };
    if (*exists.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "Alice's age:".to_string(), (*age.lock().unwrap().as_mut().unwrap()));
    }

        // Delete from map
    (*ages.lock().unwrap().as_mut().unwrap()).remove(&"Bob".to_string());
    println!("{} {}", "After deleting Bob:".to_string(), format_map(&ages));

        // Iterate over map in sorted order for deterministic output
    println!("{}", "All colors:".to_string());

        // Collect all keys into a slice
    let mut keys: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(Some(Default::default())));
    for (k, _) in (*colors.lock().unwrap().as_ref().unwrap()).clone() {
        {(*keys.lock().unwrap().as_mut().unwrap()).push(k); keys.clone()};
    }

        // Sort the keys
    (*keys.lock().unwrap().as_mut().unwrap()).sort();

        // Print in sorted order
    for k in &(*keys.lock().unwrap().as_mut().unwrap()) {
        println!("{} {} {}", k, "->".to_string(), (*(*colors.lock().unwrap().as_ref().unwrap()).get(k).unwrap().lock().unwrap().as_ref().unwrap()));
    }
}