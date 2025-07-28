pub fn say_hello(name: String) {
    let mut i = 0;
    while i < 3 {
        print!("Hello {}! ({})\n", name, i + 1);
        time.sleep(100 * time.millisecond);
        i += 1;
    }
}

pub fn counter(start: i32) {
    let mut i = start;
    while i < start + 5 {
        print!("Count: {}\n", i);
        time.sleep(50 * time.millisecond);
        i += 1;
    }
}

fn main() {
    println!("{}", "Starting goroutines...".to_string());
    
    
    
    
    time.sleep(1 * time.second);
    println!("{}", "Main function ending".to_string());
}