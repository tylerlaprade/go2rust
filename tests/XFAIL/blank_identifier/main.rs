pub fn multiple_returns() -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<bool>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some(42))), std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(true))));
}

pub fn process_slice(slice: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    let mut sum: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut count: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));

    { let new_val = 0; *sum.lock().unwrap() = Some(new_val); };
    { let new_val = (*slice.lock().unwrap().as_mut().unwrap()).len(); *count.lock().unwrap() = Some(new_val); };
    for val in &(*slice.lock().unwrap().as_mut().unwrap()) {
        { let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + val); };
    }
    return (sum, count);
}

fn main() {
    println!("{}", "=== Ignoring return values ===".to_string());
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", (*num.lock().unwrap().as_mut().unwrap()));
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", (*str.lock().unwrap().as_mut().unwrap()));
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", (*flag.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Ignoring in range loops ===".to_string());
    let mut slice = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30, 40, 50])));
    println!("{}", "Values only:".to_string());
    for val in &(*slice.lock().unwrap().as_mut().unwrap()) {
        print!("{} ", val);
    }
    println!();
    println!("{}", "Indices only:".to_string());
    for (i, _) in (*slice.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("{} ", i);
    }
    println!();
    println!("{}", "Indices (idiomatic):".to_string());
    for i in 0..(*slice.lock().unwrap().as_mut().unwrap()).len() {
        print!("{} ", i);
    }
    println!();
    println!("{}", "\n=== Ignoring in map iteration ===".to_string());
    let mut ages = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::from([("Alice".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(25)))), ("Bob".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(30)))), ("Carol".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(35))))]))));
    println!("{}", "Keys only:".to_string());
    for (name, _) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} ", name);
    }
    println!();
    println!("{}", "Values only:".to_string());
    for (_, age) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} ", (*age.lock().unwrap().as_mut().unwrap()));
    }
    println!();
    println!("{}", "\n=== Ignoring some return values in assignment ===".to_string());
    let (mut sum, _) = process_slice(slice.clone());
    print!("Sum (ignoring count): {}\n", (*sum.lock().unwrap().as_mut().unwrap()));
    let (_, mut count) = process_slice(slice.clone());
    print!("Count (ignoring sum): {}\n", (*count.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Blank identifier in declarations ===".to_string());
    let _ = "This string is assigned but not used".to_string();
    let (mut a, _, mut c) = (std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(3))));
    print!("a={}, c={} (middle value ignored)\n", (*a.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Blank identifier with type assertion ===".to_string());
    let mut value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some("hello world".to_string())));
    let (_, mut ok) = match (*value.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Value is a string (but we ignored the actual value)".to_string());
    }
    let (_, mut ok) = match (*value.lock().unwrap().as_mut().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{}", "Value is an int".to_string());
    } else {
        println!("{}", "Value is not an int".to_string());
    }
    println!("{}", "\n=== Blank identifier with channels ===".to_string());
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 3])));
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    close(ch.clone());
    for  {
        println!("{}", "Received a value (but ignored it)".to_string());
    }
    println!("{}", "\n=== Blank identifier in error handling ===".to_string());
    let (mut result, _) = process_slice(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])))))));
    print!("Result (ignoring potential error): {}\n", (*result.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Complex example ===".to_string());
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));
    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    for row in &(*data.lock().unwrap().as_mut().unwrap()) {
        for val in &row {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + val); };
    }
    }
    print!("Total of all values: {}\n", (*total.lock().unwrap().as_mut().unwrap()));
}