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
    let mut x = Arc::new(Mutex::new(Some(2)));
    match (*x.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "One".to_string());
        }
        2 => {
            println!("{}", "Two".to_string());
            // TODO: fallthrough not supported
        }
        3 => {
            println!("{}", "Three (via fallthrough)".to_string());
        }
        4 => {
            println!("{}", "Four".to_string());
        }
        _ => {
            println!("{}", "Other".to_string());
        }
    }

    println!("{}", "---".to_string());

    let mut grade = Arc::new(Mutex::new(Some('B')));
    match (*grade.lock().unwrap().as_mut().unwrap()) {
        'A' => {
            println!("{}", "Excellent!".to_string());
            // TODO: fallthrough not supported
        }
        'B' => {
            println!("{}", "Good job!".to_string());
            // TODO: fallthrough not supported
        }
        'C' => {
            println!("{}", "Passed".to_string());
        }
        'D' => {
            println!("{}", "Barely passed".to_string());
        }
        'F' => {
            println!("{}", "Failed".to_string());
        }
        _ => {}
    }

    println!("{}", "---".to_string());

    let mut n = Arc::new(Mutex::new(Some(15)));
    match true {
        true if (*n.lock().unwrap().as_mut().unwrap()) % 15 == 0 => {
            println!("{}", "FizzBuzz".to_string());
            // TODO: fallthrough not supported
        }
        true if (*n.lock().unwrap().as_mut().unwrap()) % 3 == 0 => {
            println!("{}", "Fizz".to_string());
        }
        true if (*n.lock().unwrap().as_mut().unwrap()) % 5 == 0 => {
            println!("{}", "Buzz".to_string());
        }
        _ => {
            println!("{}", (*n.lock().unwrap().as_mut().unwrap()));
        }
    }
}