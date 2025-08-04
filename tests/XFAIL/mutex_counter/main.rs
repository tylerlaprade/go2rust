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

#[derive(Debug)]
struct Counter {
    mu: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>,
    value: Arc<Mutex<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        self.mu.clone().lock();
        __defer_stack.push(Box::new(move || {
        self.mu.clone().unlock();
    }));
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> Arc<Mutex<Option<i32>>> {
        self.mu.clone().lock();
        __defer_stack.push(Box::new(move || {
        self.mu.clone().unlock();
    }));
        return self.value.clone();
    }
}

fn main() {
    let mut counter = Arc::new(Mutex::new(Some(Counter {  })));
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_mut().unwrap()));
}