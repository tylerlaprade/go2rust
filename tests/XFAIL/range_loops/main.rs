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

fn main() {
    println!("{}", "=== Range over slice ===".to_string());
    let mut numbers = Arc::new(Mutex::new(Some(vec![10, 20, 30, 40, 50])));

    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Index {}: {}\n", i, num);
    }

    println!("{}", "Values only:".to_string());
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        print!("{} ", num);
    }
    println!();

    println!("{}", "Indices only:".to_string());
    for i in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        print!("{} ", i);
    }
    println!();

    println!("{}", "\n=== Range over array ===".to_string());
    let mut arr = Arc::new(Mutex::new(Some(["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()])));
    for (i, fruit) in (*arr.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("{}: {}\n", i, fruit);
    }

    println!("{}", "\n=== Range over string ===".to_string());
    let mut text = Arc::new(Mutex::new(Some("Hello, 世界".to_string())));
    for (i, char) in (*(*text.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).chars().enumerate() {
        print!("Byte {}: {} (Unicode: {:?})\n", i, char, char);
    }

    println!("{}", "\n=== Range over map ===".to_string());
    let mut ages = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<i32>>>>::from([("Alice".to_string(), Arc::new(Mutex::new(Some(25)))), ("Bob".to_string(), Arc::new(Mutex::new(Some(30)))), ("Charlie".to_string(), Arc::new(Mutex::new(Some(35))))]))));

    for (name, age) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} is {} years old\n", name, (*age.lock().unwrap().as_mut().unwrap()));
    }

    println!("{}", "Keys only:".to_string());
    for (name, _) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} ", name);
    }
    println!();

    println!("{}", "\n=== Range over channel ===".to_string());
    let mut ch = ;

    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (close.lock().unwrap().as_ref().unwrap())(ch.clone());

    for value in 0..(*ch.lock().unwrap().as_mut().unwrap()).len() {
        print!("Received: {}\n", value);
    }

    println!("{}", "\n=== Range with break/continue ===".to_string());
    let mut data = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Even numbers only (with continue):".to_string());
    for num in &(*data.lock().unwrap().as_mut().unwrap()) {
        if num % 2 != 0 {
        continue
    }
        print!("{} ", num);
    }
    println!();

    println!("{}", "Numbers until 6 (with break):".to_string());
    for num in &(*data.lock().unwrap().as_mut().unwrap()) {
        if num > 6 {
        break
    }
        print!("{} ", num);
    }
    println!();

    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = Arc::new(Mutex::new(Some(vec![, , ])));

    for (i, row) in (*matrix.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
        print!("matrix[{}][{}] = {}\n", i, j, val);
    }
    }

    println!("{}", "\n=== Range over empty collections ===".to_string());
    let mut emptySlice: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut emptyMap: Arc<Mutex<Option<HashMap<String, i32>>>>;

    println!("{}", "Empty slice:".to_string());
    for (i, v) in (*emptySlice.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("This won't print: {}, {}\n", i, v);
    }
    println!("{}", "Empty slice range completed".to_string());

    println!("{}", "Empty map:".to_string());
    for (k, v) in (*emptyMap.lock().unwrap().as_ref().unwrap()).clone() {
        print!("This won't print: {}, {}\n", k, (*v.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "Empty map range completed".to_string());
}