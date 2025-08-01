fn main() {
    let mut x: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    let (mut s, mut ok) = match (*x.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "x is string:".to_string(), (*s.lock().unwrap().as_mut().unwrap()));
    }
    let mut str = std::sync::Arc::new(std::sync::Mutex::new(Some(match (*x.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) })));
    println!("{} {}", "Asserted string:".to_string(), (*str.lock().unwrap().as_mut().unwrap()));
    let (mut n, mut ok) = match (*x.lock().unwrap().as_mut().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        println!("{} {}", "x is int:".to_string(), (*n.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "x is not an int".to_string());
    }
}