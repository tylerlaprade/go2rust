fn main() {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(2)));
    match (*i.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "one".to_string());
        }
        2 => {
            println!("{}", "two".to_string());
        }
        3 => {
            println!("{}", "three".to_string());
        }
    }
    match true {
        true if (*i.lock().unwrap().as_mut().unwrap()) < 2 => {
            println!("{}", "less than 2".to_string());
        }
        true if (*i.lock().unwrap().as_mut().unwrap()) > 2 => {
            println!("{}", "greater than 2".to_string());
        }
        _ => {
            println!("{}", "equal to 2".to_string());
        }
    }
}