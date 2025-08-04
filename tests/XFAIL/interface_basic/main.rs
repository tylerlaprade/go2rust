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

pub fn print_any(v: Arc<Mutex<Option<Box<dyn Any>>>>) {
    println!("{} {}", "Value:".to_string(), (*v.lock().unwrap().as_mut().unwrap()));
}

fn main() {
    let mut x: Arc<Mutex<Option<Box<dyn Any>>>>;

    { let new_val = 42; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is int:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());

    { let new_val = "hello".to_string(); *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is string:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());

    { let new_val = 3.14; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x is float:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    print_any(x.clone());

    let mut values = Arc::new(Mutex::new(Some(vec![1, "two".to_string(), 3.0])));
    println!("{} {}", "Mixed values:".to_string(), format_slice(&values));
}