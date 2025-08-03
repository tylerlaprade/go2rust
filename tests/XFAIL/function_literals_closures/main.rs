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

fn main() {
    let mut add = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));
    println!("{} {}", "add(3, 4) =".to_string(), (*(add.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4)))).lock().unwrap().as_mut().unwrap()));

    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    let mut increment = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));
    println!("{} {}", "increment() =".to_string(), (*(increment.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "increment() =".to_string(), (*(increment.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

    let mut makeMultiplier = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |factor: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |x: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*factor.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>>>> + Send + Sync>))))));
    let mut double = make_multiplier(std::sync::Arc::new(std::sync::Mutex::new(Some(2))));
    let mut triple = make_multiplier(std::sync::Arc::new(std::sync::Mutex::new(Some(3))));
    println!("{} {}", "double(5) =".to_string(), (*(double.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "triple(5) =".to_string(), (*(triple.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));

    let mut result = (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))).lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    println!("{} {}", "IIFE result =".to_string(), (*result.lock().unwrap().as_mut().unwrap()));

    let mut operations = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) - (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)))])));

    for (i, op) in (*operations.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, (*(op.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    }
}