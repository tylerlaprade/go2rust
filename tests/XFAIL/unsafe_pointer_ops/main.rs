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
    x: std::sync::Arc<std::sync::Mutex<Option<int32>>>,
    y: std::sync::Arc<std::sync::Mutex<Option<int32>>>,
}

fn main() {
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(Point { x: std::sync::Arc::new(std::sync::Mutex::new(Some(10))), y: std::sync::Arc::new(std::sync::Mutex::new(Some(20))) })));

    let mut xPtr = ((*int32.lock().unwrap().as_mut().unwrap()))(std::sync::Arc::new(std::sync::Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap())))))))));
    print!("X via unsafe: {}\n", (*xPtr.lock().unwrap().as_mut().unwrap()));

    let mut yPtr = ((*int32.lock().unwrap().as_mut().unwrap()))(std::sync::Arc::new(std::sync::Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(std::sync::Arc::new(std::sync::Mutex::new(Some(uintptr(std::sync::Arc::new(std::sync::Mutex::new(Some((*unsafe.lock().unwrap().as_mut().unwrap()).pointer(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))))))) + (*unsafe.lock().unwrap().as_mut().unwrap()).offsetof(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()).y))))))))))));
    print!("Y via unsafe: {}\n", (*yPtr.lock().unwrap().as_mut().unwrap()));

    print!("Size: {}, Align: {}\n", (*(*unsafe.lock().unwrap().as_mut().unwrap()).sizeof(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))).lock().unwrap().as_mut().unwrap()), (*(*unsafe.lock().unwrap().as_mut().unwrap()).alignof(std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_mut().unwrap()))))).lock().unwrap().as_mut().unwrap()));
}