#[derive(Debug)]
struct Config {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    version: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    debug: std::sync::Arc<std::sync::Mutex<Option<bool>>>,
}

pub fn init() {
    println!("{}", "First init function called".to_string());
    { let new_val = 10; *globalCounter.lock().unwrap() = Some(new_val); };
    { let new_val = true; *initialized.lock().unwrap() = Some(new_val); };
}

pub fn init() {
    println!("{}", "Second init function called".to_string());
    (*globalCounter.lock().unwrap().as_ref().unwrap()) += 5;
    { let new_val = std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<String>>>>::new(); *configData.lock().unwrap() = Some(new_val); };
    (*configData.lock().unwrap().as_mut().unwrap())["version".to_string()] = "1.0".to_string();
    (*configData.lock().unwrap().as_mut().unwrap())["author".to_string()] = "go2rust".to_string();
}

pub fn init() {
    println!("{}", "Third init function called".to_string());
    if (*initialized.lock().unwrap().as_ref().unwrap()) {
        print!("Global counter initialized to: {}\n", (*globalCounter.lock().unwrap().as_ref().unwrap()));
    }
    (*configData.lock().unwrap().as_mut().unwrap())["build".to_string()] = "debug".to_string();
    (*configData.lock().unwrap().as_mut().unwrap())["target".to_string()] = "rust".to_string();
}

pub fn compute_initial_value() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    println!("{}", "Computing initial value during package initialization".to_string());
    return std::sync::Arc::new(std::sync::Mutex::new(Some(42 * 2)));
}

pub fn init() {
    println!("{}", "Fourth init function called".to_string());
    print!("Computed value is: {}\n", (*computedValue.lock().unwrap().as_ref().unwrap()));
    (*computedValue.lock().unwrap().as_ref().unwrap()) += 10;
}

pub fn init() {
    println!("{}", "Fifth init function - initializing app config".to_string());
    { let new_val = Config { name: "Go2Rust Transpiler".to_string(), version: "0.1.0".to_string(), debug: true }; *appConfig.lock().unwrap() = Some(new_val); };
}

pub fn init() {
    println!("{}", "Sixth init function - with potential panic handling".to_string());
    // defer () // TODO: defer not yet supported
    if false {
        panic(std::sync::Arc::new(std::sync::Mutex::new(Some("Init function panic!".to_string()))));
    }
    println!("{}", "Sixth init function completed successfully".to_string());
}

pub fn setup_logging() {
    println!("{}", "Setting up logging system...".to_string());
}

pub fn init() {
    println!("{}", "Seventh init function - setting up subsystems".to_string());
    setup_logging();
    if (*configData.lock().unwrap().as_ref().unwrap()).len() == 0 {
        println!("{}", "Warning: No configuration data found".to_string());
    } else {
        print!("Configuration loaded with {} entries\n", (*configData.lock().unwrap().as_ref().unwrap()).len());
    }
}

fn main() {
    println!("{}", "\n=== Main function started ===".to_string());
    print!("Global counter: {}\n", (*globalCounter.lock().unwrap().as_ref().unwrap()));
    print!("Initialized flag: {}\n", (*initialized.lock().unwrap().as_ref().unwrap()));
    print!("Computed value: {}\n", (*computedValue.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\nConfiguration data:".to_string());
    for (key, value) in (*configData.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("  {}: {}\n", key, value);
    }
    print!("\nApp config: %+v\n", (*appConfig.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Calling functions that were used in init ===".to_string());
    print!("Calling computeInitialValue() again: {}\n", compute_initial_value());
    setup_logging();
    print!("Global counter still: {}\n", (*globalCounter.lock().unwrap().as_ref().unwrap()));
    { let new_val = 100; *globalCounter.lock().unwrap() = Some(new_val); };
    print!("Modified global counter: {}\n", (*globalCounter.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Demonstrating init execution order ===".to_string());
    println!("{}", "1. Package-level variable declarations".to_string());
    println!("{}", "2. Package-level variable initializations (like computedValue)".to_string());
    println!("{}", "3. Init functions in the order they appear in source".to_string());
    println!("{}", "4. Main function".to_string());
    println!("{}", "\n=== Main function completed ===".to_string());
}