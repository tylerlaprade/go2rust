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

pub fn make_counter() -> std::sync::Arc<std::sync::Mutex<Option<Box<dyn Fn() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>>>> {

    let mut count = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    return std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return count.clone();
    }) as Box<dyn Fn() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn make_adder(x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |y: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) + (*y.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn apply_operation(nums: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, op: std::sync::Arc<std::sync::Mutex<Option<Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; (*nums.lock().unwrap().as_mut().unwrap()).len()])));
    for (i, num) in (*nums.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[i] = (op.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(num))));
    }
    return result.clone();
}

fn main() {
    let mut counter = make_counter();
    println!("{} {}", "Counter 1:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Counter 2:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Counter 3:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));

    let mut counter2 = make_counter();
    println!("{} {}", "Counter2 1:".to_string(), (*(counter2.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Counter 4:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_mut().unwrap()));

    let mut add5 = make_adder(std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    let mut add10 = make_adder(std::sync::Arc::new(std::sync::Mutex::new(Some(10))));

    println!("{} {}", "5 + 3 =".to_string(), (*(add5.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "10 + 7 =".to_string(), (*(add10.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(7)))).lock().unwrap().as_mut().unwrap()));

    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])));

    let mut squared = apply_operation(numbers.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |x: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*x.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)))))));
    println!("{} {}", "Squared:".to_string(), format_slice(&squared));

    let mut doubled = apply_operation(numbers.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |x: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * 2)));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)))))));
    println!("{} {}", "Doubled:".to_string(), format_slice(&doubled));

    let mut multiplier = std::sync::Arc::new(std::sync::Mutex::new(Some(3)));
    let mut tripled = apply_operation(numbers.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |x: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*multiplier.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>)))))));
    println!("{} {}", "Tripled:".to_string(), format_slice(&tripled));

    let mut result = (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))).lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20))));
    println!("{} {}", "Immediate result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
}