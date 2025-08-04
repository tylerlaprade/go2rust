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

const FLAG_READ: i64 = 1 << 0;
const FLAG_WRITE: i64 = 1 << 1;
const FLAG_EXECUTE: i64 = 1 << 2;
const FLAG_DELETE: i64 = 1 << 3;


const K_B: i64 = 1 << (10 * 1);
const M_B: i64 = 1 << (10 * 2);
const G_B: i64 = 1 << (10 * 3);
const T_B: i64 = 1 << (10 * 4);


const A: i32 = 0;
const B: i32 = 0 * 10;
const C: i32 = 1;
const D: i32 = 1;
const E: i32 = 2;
const F: i32 = 2;


const FIRST: i32 = 0;
const SECOND: i32 = 1;


const THIRD: i32 = 0;
const FOURTH: i32 = 1;


fn main() {
    let mut perms = Arc::new(Mutex::new(Some(FLAG_READ | FLAG_WRITE)));
    print!("Permissions: {} (Read={}, Write={})\n", (*perms.lock().unwrap().as_mut().unwrap()), FLAG_READ, FLAG_WRITE);

    print!("KB={}, MB={}, GB={}\n", K_B, M_B, G_B);

    print!("A={}, B={}, C={}, D={}, E={}, F={}\n", A, B, C, D, E, F);

    print!("First={}, Second={}, Third={}, Fourth={}\n", FIRST, SECOND, THIRD, FOURTH);
}