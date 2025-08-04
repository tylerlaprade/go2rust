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

pub fn f() -> Arc<Mutex<Option<i32>>> {

    { let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return d.clone();
}

pub fn init() {
    println!("{}", "First init".to_string());
    { let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

pub fn init() {
    println!("{}", "Second init".to_string());
    print!("a={}, b={}, c={}, d={}\n", (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()), (*d.lock().unwrap().as_mut().unwrap()));
}

fn main() {
    print!("In main: a={}, b={}, c={}, d={}\n", (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()), (*d.lock().unwrap().as_mut().unwrap()));
}