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
    let mut add = Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))))));
    println!("{} {}", "add(3, 4) =".to_string(), (*(add.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(4)))).lock().unwrap().as_mut().unwrap()));

    let mut x = Arc::new(Mutex::new(Some(10)));
    let mut increment = Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return x.clone();
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>))))));
    println!("{} {}", "increment() =".to_string(), (*(increment.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "increment() =".to_string(), (*(increment.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

    let mut makeMultiplier = Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |factor: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {
        return Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*factor.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> + Send + Sync>))))));
    let mut double = (makeMultiplier.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(2))));
    let mut triple = (makeMultiplier.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3))));
    println!("{} {}", "double(5) =".to_string(), (*(double.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "triple(5) =".to_string(), (*(triple.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));

    let mut result = (Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(5))));
    println!("{} {}", "IIFE result =".to_string(), (*result.lock().unwrap().as_mut().unwrap()));

    let mut operations = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) - (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))])));

    for (i, op) in (*operations.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, (*(op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    }
}