use std::sync::{Arc, Mutex};

fn main() {
        // Basic function literal
    let mut add = Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))))));
    println!("{} {}", "add(3, 4) =".to_string(), (*(add.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(4)))).lock().unwrap().as_ref().unwrap()));

        // Closure capturing variables
    let mut x = Arc::new(Mutex::new(Some(10)));
    let mut increment = Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return x.clone();
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>))))));
    println!("{} {}", "increment() =".to_string(), (*(increment.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "increment() =".to_string(), (*(increment.lock().unwrap().as_ref().unwrap())().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

        // Function returning closure
    let mut makeMultiplier = Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |factor: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {
        return Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*factor.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> + Send + Sync>))))));
    let mut double = (makeMultiplier.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(2))));
    let mut triple = (makeMultiplier.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(3))));
    println!("{} {}", "double(5) =".to_string(), (*(double.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "triple(5) =".to_string(), (*(triple.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));

        // Immediately invoked function
    let mut result = (Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(5))));
    println!("{} {}", "IIFE result =".to_string(), (*result.lock().unwrap().as_mut().unwrap()));

        // Function literal in slice
    let mut operations = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))])));

    for (i, op) in (*operations.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, (*(op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));
    }
}