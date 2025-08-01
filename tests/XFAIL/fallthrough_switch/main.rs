fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(2)));
    match (*x.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "One".to_string());
        }
        2 => {
            println!("{}", "Two".to_string());
            // TODO: fallthrough not supported
        }
        3 => {
            println!("{}", "Three (via fallthrough)".to_string());
        }
        4 => {
            println!("{}", "Four".to_string());
        }
        _ => {
            println!("{}", "Other".to_string());
        }
    }
    println!("{}", "---".to_string());
    let mut grade = std::sync::Arc::new(std::sync::Mutex::new(Some('B')));
    match (*grade.lock().unwrap().as_mut().unwrap()) {
        'A' => {
            println!("{}", "Excellent!".to_string());
            // TODO: fallthrough not supported
        }
        'B' => {
            println!("{}", "Good job!".to_string());
            // TODO: fallthrough not supported
        }
        'C' => {
            println!("{}", "Passed".to_string());
        }
        'D' => {
            println!("{}", "Barely passed".to_string());
        }
        'F' => {
            println!("{}", "Failed".to_string());
        }
        _ => {}
    }
    println!("{}", "---".to_string());
    let mut n = std::sync::Arc::new(std::sync::Mutex::new(Some(15)));
    match true {
        true if (*n.lock().unwrap().as_mut().unwrap()) % 15 == 0 => {
            println!("{}", "FizzBuzz".to_string());
            // TODO: fallthrough not supported
        }
        true if (*n.lock().unwrap().as_mut().unwrap()) % 3 == 0 => {
            println!("{}", "Fizz".to_string());
        }
        true if (*n.lock().unwrap().as_mut().unwrap()) % 5 == 0 => {
            println!("{}", "Buzz".to_string());
        }
        _ => {
            println!("{}", (*n.lock().unwrap().as_mut().unwrap()));
        }
    }
}