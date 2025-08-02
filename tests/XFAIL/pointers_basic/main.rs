fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
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
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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
struct Point {
    x: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    y: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", "Value of x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Address of x:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Value through pointer:".to_string(), (*p.lock().unwrap().as_mut().unwrap()));
    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Modified x:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    let mut point = std::sync::Arc::new(std::sync::Mutex::new(Some(Point { x: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), y: std::sync::Arc::new(std::sync::Mutex::new(Some(20))) })));
    println!("{} {}", "Point:".to_string(), (*point.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Point X:".to_string(), (*point.lock().unwrap().as_mut().unwrap()).x);
    println!("{} {}", "Point Y:".to_string(), (*point.lock().unwrap().as_mut().unwrap()).y);
    { let new_val = 30; *(*point.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 40; *(*point.lock().unwrap().as_mut().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Modified point:".to_string(), (*point.lock().unwrap().as_mut().unwrap()));
    let mut q = std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))));
    { let new_val = 200; *q.lock().unwrap() = Some(new_val); };
    println!("{} {}", "x after modifying through q:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    let mut newPoint = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc<std::sync::Mutex<Option<Point>>>::default())));
    { let new_val = 5; *(*newPoint.lock().unwrap().as_mut().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 15; *(*newPoint.lock().unwrap().as_mut().unwrap()).y.lock().unwrap() = Some(new_val); };
    println!("{} {}", "New point:".to_string(), (*newPoint.lock().unwrap().as_mut().unwrap()));
}