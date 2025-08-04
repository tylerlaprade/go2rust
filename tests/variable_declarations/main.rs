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
    let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
    let mut y: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some("hello".to_string())));
    let mut z: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(3.14)));

    let mut a = Arc::new(Mutex::new(Some(100)));
    let mut b = Arc::new(Mutex::new(Some("world".to_string())));
    let mut c = Arc::new(Mutex::new(Some(2.71)));

    println!("{} {} {} {}", "Variables:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()), (*z.lock().unwrap().as_mut().unwrap()));
    println!("{} {} {} {}", "Short vars:".to_string(), (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()));
}