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
struct List {
    head: Arc<Mutex<Option</* TODO: Unhandled type *ast.IndexExpr */ Arc<Mutex<Option<()>>>>>>,
    tail: Arc<Mutex<Option</* TODO: Unhandled type *ast.IndexExpr */ Arc<Mutex<Option<()>>>>>>,
}

#[derive(Debug)]
struct element {
    next: Arc<Mutex<Option</* TODO: Unhandled type *ast.IndexExpr */ Arc<Mutex<Option<()>>>>>>,
    val: Arc<Mutex<Option<T>>>,
}

impl Unknown {
    pub fn push(&mut self, v: Arc<Mutex<Option<T>>>) {
        if (*self.tail.lock().unwrap()).is_none() {
        { let new_val = (*.lock().unwrap()).clone(); *self.head.lock().unwrap() = new_val; };
        { let new_val = self.head.clone(); *self.tail.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = (*.lock().unwrap()).clone(); *self.tail.clone().next.lock().unwrap() = new_val; };
        { let new_val = self.tail.clone().next; *self.tail.lock().unwrap() = Some(new_val); };
    }
    }
}

pub fn map_keys(m: Arc<Mutex<Option<HashMap<K, V>>>>) -> Arc<Mutex<Option<Vec<K>>>> {

    let mut r = Arc::new(Mutex::new(Some(Vec::with_capacity((*m.lock().unwrap().as_mut().unwrap()).len()))));
    for (k, _) in (*m.lock().unwrap().as_ref().unwrap()).clone() {
        {(*r.lock().unwrap().as_mut().unwrap()).push(k); r.clone()};
    }
    return r.clone();
}

fn main() {
    let mut m = Arc::new(Mutex::new(Some(HashMap::<i32, Arc<Mutex<Option<String>>>>::from([(1, Arc::new(Mutex::new(Some("2".to_string())))), (2, Arc::new(Mutex::new(Some("4".to_string())))), (4, Arc::new(Mutex::new(Some("8".to_string()))))]))));
    println!("{} {}", "keys:".to_string(), format_slice(&map_keys(m.clone())));

    let mut lst = ;
    (*lst.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(10))));
    (*lst.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(13))));
    (*lst.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(23))));
}