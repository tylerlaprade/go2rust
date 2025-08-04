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
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some("hello".to_string())));

    let (mut s, mut ok) = match (*x.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "x is string:".to_string(), (*s.lock().unwrap().as_mut().unwrap()));
    }

    let mut str = Arc::new(Mutex::new(Some(match (*x.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) })));
    println!("{} {}", "Asserted string:".to_string(), (*str.lock().unwrap().as_mut().unwrap()));

    let (mut n, mut ok) = match (*x.lock().unwrap().as_mut().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "x is int:".to_string(), (*n.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "x is not an int".to_string());
    }
}