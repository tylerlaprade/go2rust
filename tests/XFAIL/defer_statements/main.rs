pub fn defer_example() {
    println!("{}", "Start of function".to_string());
    
    
    
    println!("{}", "Middle of function".to_string());
    
    println!("{}", "End of function".to_string());
}

pub fn defer_with_variables() {
    let mut x = 10;
    
    x = 20;
    println!("{} {}", "Current x:".to_string(), x);
}

pub fn defer_in_loop() {
    println!("{}", "Defer in loop:".to_string());
    let mut i = 0;
    while i < 3 {
        
        i += 1;
    }
    println!("{}", "Loop finished".to_string());
}

pub fn cleanup() {
    println!("{}", "Cleanup function called".to_string());
}

pub fn resource_example() {
    println!("{}", "Acquiring resource".to_string());
    
    println!("{}", "Using resource".to_string());
    let mut i = 0;
    while i < 3 {
        print!("Working... {}\n", i + 1);
        i += 1;
    }
    println!("{}", "Done with resource".to_string());
}

fn main() {
    println!("{}", "=== Basic defer example ===".to_string());
    defer_example();
    println!("{}", "\n=== Defer with variables ===".to_string());
    defer_with_variables();
    println!("{}", "\n=== Defer in loop ===".to_string());
    defer_in_loop();
    println!("{}", "\n=== Resource cleanup example ===".to_string());
    resource_example();
    println!("{}", "\n=== Main function ending ===".to_string());
}