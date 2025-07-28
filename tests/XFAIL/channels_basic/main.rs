pub fn sender(ch: Unknown) {
    let mut i = 1;
    while i <= 5 {
        print!("Sending: {}\n", i);
        
        time.sleep(100 * time.millisecond);
        i += 1;
    }
    close(ch);
}

pub fn receiver(ch: Unknown) {
    while true {
        let mut value, let mut ok = ;
        
        print!("Received: {}\n", value);
    }
}

fn main() {
    let mut ch = ;
    
    
    time.sleep(1 * time.second);
    let mut buffered = vec![0; 3];
    
    
    
    println!("{}", "Buffered channel contents:".to_string());
    let mut i = 0;
    while i < 3 {
        let mut msg = ;
        println!("{} {}", "Got:".to_string(), msg);
        i += 1;
    }
    let mut numbers = vec![0; 5];
    
    println!("{}", "Range over channel:".to_string());
    for num in 0..numbers.len() {
        println!("{} {}", "Number:".to_string(), num);
    }
}