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

pub fn worker(id: Arc<Mutex<Option<i32>>>, wg: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        (*wg.lock().unwrap().as_mut().unwrap()).done();
    }));
    print!("Worker {} starting\n", (*id.lock().unwrap().as_mut().unwrap()));
    (*time.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some((*time.lock().unwrap().as_mut().unwrap())::second))));
    print!("Worker {} done\n", (*id.lock().unwrap().as_mut().unwrap()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

fn main() {
    let mut wg: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>;
    let mut i = Arc::new(Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 3 {
        (*wg.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1))));
        // TODO: Unhandled statement type: GoStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*wg.lock().unwrap().as_mut().unwrap()).wait();
    println!("{}", "All workers done".to_string());
}