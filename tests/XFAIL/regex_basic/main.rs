fn main() {
    let mut pattern = std::sync::Arc::new(std::sync::Mutex::new(Some(`\d+`.to_string())));
    let mut re = (*regexp.lock().unwrap().as_mut().unwrap()).must_compile(std::sync::Arc::new(std::sync::Mutex::new(Some((*pattern.lock().unwrap().as_mut().unwrap())))));
    let mut text = std::sync::Arc::new(std::sync::Mutex::new(Some("I have 42 apples and 7 oranges".to_string())));
    let mut matches = (*re.lock().unwrap().as_mut().unwrap()).find_all_string(std::sync::Arc::new(std::sync::Mutex::new(Some((*text.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(-1))));
    println!("{} {}", "Numbers found:".to_string(), (*matches.lock().unwrap().as_mut().unwrap()));
}