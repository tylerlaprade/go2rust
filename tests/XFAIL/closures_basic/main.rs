pub fn make_counter() -> std::sync::Arc<std::sync::Mutex<Option<Unknown>>> {

    let mut count = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    return std::sync::Arc::new(std::sync::Mutex::new(Some()));
}

pub fn make_adder(x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<Unknown>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some()));
}

pub fn apply_operation(nums: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, op: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; (*nums.lock().unwrap().as_ref().unwrap()).len()])));
    for (i, num) in (*nums.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[(*i.lock().unwrap().as_ref().unwrap())] = op(std::sync::Arc::new(std::sync::Mutex::new(Some((*num.lock().unwrap().as_ref().unwrap())))));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_ref().unwrap()))));
}

fn main() {
    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(make_counter())));
    println!("{} {}", "Counter 1:".to_string(), (*counter().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Counter 2:".to_string(), (*counter().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Counter 3:".to_string(), (*counter().lock().unwrap().as_ref().unwrap()));
    let mut counter2 = std::sync::Arc::new(std::sync::Mutex::new(Some(make_counter())));
    println!("{} {}", "Counter2 1:".to_string(), (*counter2().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Counter 4:".to_string(), (*counter().lock().unwrap().as_ref().unwrap()));
    let mut add5 = std::sync::Arc::new(std::sync::Mutex::new(Some(make_adder(std::sync::Arc::new(std::sync::Mutex::new(Some(5)))))));
    let mut add10 = std::sync::Arc::new(std::sync::Mutex::new(Some(make_adder(std::sync::Arc::new(std::sync::Mutex::new(Some(10)))))));
    println!("{} {}", "5 + 3 =".to_string(), (*add5(std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "10 + 7 =".to_string(), (*add10(std::sync::Arc::new(std::sync::Mutex::new(Some(7)))).lock().unwrap().as_ref().unwrap()));
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])));
    let mut squared = std::sync::Arc::new(std::sync::Mutex::new(Some(apply_operation(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some()))))));
    println!("{} {}", "Squared:".to_string(), (*squared.lock().unwrap().as_ref().unwrap()));
    let mut doubled = std::sync::Arc::new(std::sync::Mutex::new(Some(apply_operation(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some()))))));
    println!("{} {}", "Doubled:".to_string(), (*doubled.lock().unwrap().as_ref().unwrap()));
    let mut multiplier = std::sync::Arc::new(std::sync::Mutex::new(Some(3)));
    let mut tripled = std::sync::Arc::new(std::sync::Mutex::new(Some(apply_operation(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some()))))));
    println!("{} {}", "Tripled:".to_string(), (*tripled.lock().unwrap().as_ref().unwrap()));
    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some((std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20)))))));
    println!("{} {}", "Immediate result:".to_string(), (*result.lock().unwrap().as_ref().unwrap()));
}