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

pub fn make_counter() -> Arc<Mutex<Option<Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {

    let mut count = Arc::new(Mutex::new(Some(0)));
    return let count_captured = count.clone(); Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        { let mut guard = count_captured.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return count.clone();
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn make_adder(x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {

    return let x_captured = x.clone(); Arc::new(Mutex::new(Some(Box::new(move |y: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*x_captured.lock().unwrap().as_mut().unwrap()) + (*y.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn apply_operation(nums: Arc<Mutex<Option<Vec<i32>>>>, op: Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i32>>>> {

    let mut result = Arc::new(Mutex::new(Some(vec![0; (*nums.lock().unwrap().as_mut().unwrap()).len()])));
    for (i, num) in (*nums.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[i] = (op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(num))));
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

    let mut add5 = make_adder(Arc::new(Mutex::new(Some(5))));
    let mut add10 = make_adder(Arc::new(Mutex::new(Some(10))));

    println!("{} {}", "5 + 3 =".to_string(), (*(add5.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "10 + 7 =".to_string(), (*(add10.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(7)))).lock().unwrap().as_mut().unwrap()));

    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])));

    let mut squared = apply_operation(numbers.clone(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*x.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))))));
    println!("{} {}", "Squared:".to_string(), format_slice(&squared));

    let mut doubled = apply_operation(numbers.clone(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * 2)));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))))));
    println!("{} {}", "Doubled:".to_string(), format_slice(&doubled));

    let mut multiplier = Arc::new(Mutex::new(Some(3)));
    let mut tripled = apply_operation(numbers.clone(), Arc::new(Mutex::new(Some(let multiplier_captured = multiplier.clone(); Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*multiplier_captured.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))))));
    println!("{} {}", "Tripled:".to_string(), format_slice(&tripled));

    let mut result = (Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20))));
    println!("{} {}", "Immediate result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
}