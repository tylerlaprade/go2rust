pub fn init() {
    println!("{}", "First init function called".to_string());
    globalCounter = 10;
    initialized = true;
}

pub fn init() {
    println!("{}", "Second init function called".to_string());
    globalCounter.push_str(&5);
    configData = std::collections::HashMap::<String, String>::new();
    configData["version".to_string()] = "1.0".to_string();
    configData["author".to_string()] = "go2rust".to_string();
}

pub fn init() {
    println!("{}", "Third init function called".to_string());
    
    configData["build".to_string()] = "debug".to_string();
    configData["target".to_string()] = "rust".to_string();
}

pub fn compute_initial_value() -> i32 {
    println!("{}", "Computing initial value during package initialization".to_string());
    return 42 * 2;
}

pub fn init() {
    println!("{}", "Fourth init function called".to_string());
    print!("Computed value is: {}\n", computedValue);
    computedValue.push_str(&10);
}

#[derive(Debug)]
struct Config {
    name: String,
    version: String,
    debug: bool,
}

pub fn init() {
    println!("{}", "Fifth init function - initializing app config".to_string());
    appConfig = Config { name: "Go2Rust Transpiler".to_string(), version: "0.1.0".to_string(), debug: true };
}

pub fn init() {
    println!("{}", "Sixth init function - with potential panic handling".to_string());
    
    
    println!("{}", "Sixth init function completed successfully".to_string());
}

pub fn setup_logging() {
    println!("{}", "Setting up logging system...".to_string());
}

pub fn init() {
    println!("{}", "Seventh init function - setting up subsystems".to_string());
    setup_logging();
    
}

fn main() {
    println!("{}", "\n=== Main function started ===".to_string());
    print!("Global counter: {}\n", globalCounter);
    print!("Initialized flag: %t\n", initialized);
    print!("Computed value: {}\n", computedValue);
    println!("{}", "\nConfiguration data:".to_string());
    for (key, value) in configData.iter().enumerate() {
        print!("  {}: {}\n", key, value);
    }
    print!("\nApp config: %+v\n", appConfig);
    println!("{}", "\n=== Calling functions that were used in init ===".to_string());
    print!("Calling computeInitialValue() again: {}\n", compute_initial_value());
    setup_logging();
    print!("Global counter still: {}\n", globalCounter);
    globalCounter = 100;
    print!("Modified global counter: {}\n", globalCounter);
    println!("{}", "\n=== Demonstrating init execution order ===".to_string());
    println!("{}", "1. Package-level variable declarations".to_string());
    println!("{}", "2. Package-level variable initializations (like computedValue)".to_string());
    println!("{}", "3. Init functions in the order they appear in source".to_string());
    println!("{}", "4. Main function".to_string());
    println!("{}", "\n=== Main function completed ===".to_string());
}