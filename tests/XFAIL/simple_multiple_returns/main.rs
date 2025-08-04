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

pub fn divmod(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) {

    return (Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap())))));
}

pub fn swap(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {

    return (b.clone(), a.clone());
}

fn main() {
    let (mut q, mut r) = divmod(Arc::new(Mutex::new(Some(17))), Arc::new(Mutex::new(Some(5))));
    println!("{} {} {} {}", "Quotient:".to_string(), (*q.lock().unwrap().as_mut().unwrap()), "Remainder:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));

    let (mut x, mut y) = (Arc::new(Mutex::new(Some("hello".to_string()))), Arc::new(Mutex::new(Some("world".to_string()))));
    println!("{} {} {}", "Before swap:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()));

    (x, y) = swap(x.clone(), y.clone());
    println!("{} {} {}", "After swap:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()));

    let (_, mut r2) = divmod(Arc::new(Mutex::new(Some(23))), Arc::new(Mutex::new(Some(7))));
    println!("{} {}", "23 mod 7 =".to_string(), (*r2.lock().unwrap().as_mut().unwrap()));
}