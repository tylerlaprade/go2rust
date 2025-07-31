fn main() {
    if 7 % 2 == 0 {
        println!("{}", "7 is even".to_string());
    } else {
        println!("{}", "7 is odd".to_string());
    }
    if 8 % 4 == 0 {
        println!("{}", "8 is divisible by 4".to_string());
    }
    let mut num = std::sync::Arc::new(std::sync::Mutex::new(Some(9)));
    if (*num.lock().unwrap().as_ref().unwrap()) < 0 {
        println!("{} {}", (*num.lock().unwrap().as_ref().unwrap()), "is negative".to_string());
    } else if (*num.lock().unwrap().as_ref().unwrap()) < 10 {
        println!("{} {}", (*num.lock().unwrap().as_ref().unwrap()), "has 1 digit".to_string());
    } else {
        println!("{} {}", (*num.lock().unwrap().as_ref().unwrap()), "has multiple digits".to_string());
    }
}