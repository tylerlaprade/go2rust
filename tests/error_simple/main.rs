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

pub fn divide(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        return (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return (Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(None)));
}

fn main() {
    let (mut result, mut err) = divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }

    (result, err) = divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }
}