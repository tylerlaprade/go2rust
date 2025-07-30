pub fn sender(ch: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_ref().unwrap()) <= 5 {
        print!("Sending: {}\n", (*i.lock().unwrap().as_ref().unwrap()));
        
        (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(100 * (*time.lock().unwrap().as_ref().unwrap()).millisecond))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(std::sync::Arc::new(std::sync::Mutex::new(Some((*ch.lock().unwrap().as_ref().unwrap())))));
}

pub fn receiver(ch: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>) {
    while true {
        let (mut (*value.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = <-(*ch.lock().unwrap().as_ref().unwrap());
        if !(*ok.lock().unwrap().as_ref().unwrap()) {
        println!("{}", "Channel closed".to_string());
        
    }
        print!("Received: {}\n", (*value.lock().unwrap().as_ref().unwrap()));
    }
}

fn main() {
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    
    
    (*time.lock().unwrap().as_ref().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(1 * (*time.lock().unwrap().as_ref().unwrap()).second))));
    let mut buffered = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 3])));
    
    
    
    println!("{}", "Buffered channel contents:".to_string());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < 3 {
        let mut msg = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*buffered.lock().unwrap().as_ref().unwrap()))));
        println!("{} {}", "Got:".to_string(), (*msg.lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 5])));
    
    println!("{}", "Range over channel:".to_string());
    for num in 0..(*numbers.lock().unwrap().as_ref().unwrap()).len() {
        println!("{} {}", "Number:".to_string(), num);
    }
}