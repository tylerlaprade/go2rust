use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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
    return Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return count.clone();
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn make_adder(x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {

    return Arc::new(Mutex::new(Some(Box::new(move |y: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*y.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn apply_operation(nums: Arc<Mutex<Option<Vec<i32>>>>, op: Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<i32>>>> {

    let mut result = Arc::new(Mutex::new(Some(vec![0; (*nums.lock().unwrap().as_ref().unwrap()).len()])));
    for (i, num) in (*nums.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[i] = (op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(num))));
    }
    return result.clone();
}

fn main() {
        // Basic closure
    let mut counter = make_counter();
    println!("{} {}", "Counter 1:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Counter 2:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Counter 3:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));

        // Another counter instance
    let mut counter2 = make_counter();
    println!("{} {}", "Counter2 1:".to_string(), (*(counter2.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Counter 4:".to_string(), (*(counter.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));

        // Closure with parameters
    let mut add5 = make_adder(Arc::new(Mutex::new(Some(5))));
    let mut add10 = make_adder(Arc::new(Mutex::new(Some(10))));

    println!("{} {}", "5 + 3 =".to_string(), (*(add5.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "10 + 7 =".to_string(), (*(add10.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(7)))).lock().unwrap().as_ref().unwrap()));

        // Higher-order functions
    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])));

        // Square function
    let mut squared = apply_operation(numbers.clone(), Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*x.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))));
    println!("{} {}", "Squared:".to_string(), format_slice(&squared));

        // Double function
    let mut doubled = apply_operation(numbers.clone(), Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = 2;
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))));
    println!("{} {}", "Doubled:".to_string(), format_slice(&doubled));

        // Closure capturing local variable
    let mut multiplier = Arc::new(Mutex::new(Some(3)));
    let mut tripled = apply_operation(numbers.clone(), Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*multiplier.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))));
    println!("{} {}", "Tripled:".to_string(), format_slice(&tripled));

        // Immediately invoked function
    let mut result = (Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20))));
    println!("{} {}", "Immediate result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
}