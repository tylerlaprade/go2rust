pub fn basic_select() {
    let mut ch1 = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut ch2 = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    
    
    
}

pub fn select_with_timeout() {
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    
    
}

pub fn select_with_default() {
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 1])));
    
    
    
}

pub fn select_loop() {
    let mut ch1 = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut ch2 = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut quit = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    
    
    
    println!("{}", "Starting select loop:".to_string());
    while true {
        
    }
}

pub fn select_with_send() {
    let mut ch1 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 1])));
    let mut ch2 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 1])));
    
    println!("{} {}", "Reading from ch1:".to_string(), <-(*ch1.lock().unwrap().as_ref().unwrap()));
    
}

fn main() {
    println!("{}", "=== Basic select ===".to_string());
    basic_select();
    println!("{}", "\n=== Select with timeout ===".to_string());
    select_with_timeout();
    println!("{}", "\n=== Select with default ===".to_string());
    select_with_default();
    println!("{}", "\n=== Select with send ===".to_string());
    select_with_send();
    println!("{}", "\n=== Select loop ===".to_string());
    select_loop();
    println!("{}", "\n=== All examples completed ===".to_string());
}