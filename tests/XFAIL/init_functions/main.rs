use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<HashMap<K, Arc<Mutex<Option<V>>>>>>>) -> String 
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

/// Struct with initialization
#[derive(Debug)]
struct Config {
    name: Arc<Mutex<Option<String>>>,
    version: Arc<Mutex<Option<String>>>,
    debug: Arc<Mutex<Option<bool>>>,
}

/// First init function
pub fn init() {
    println!("{}", "First init function called".to_string());
    { let new_val = 10; *globalCounter.lock().unwrap() = Some(new_val); };
    { let new_val = true; *initialized.lock().unwrap() = Some(new_val); };
}

/// Second init function (they run in order)
pub fn init() {
    println!("{}", "Second init function called".to_string());
    { let mut guard = globalCounter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 5); };

    { let new_val = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<String>>>>::new()))); *configData.lock().unwrap() = Some(new_val); };
    (*configData.lock().unwrap().as_mut().unwrap())["version".to_string()] = "1.0".to_string();
    (*configData.lock().unwrap().as_mut().unwrap())["author".to_string()] = "go2rust".to_string();
}

/// Third init function
pub fn init() {
    println!("{}", "Third init function called".to_string());
    if (*initialized.lock().unwrap().as_mut().unwrap()) {
        print!("Global counter initialized to: {}\n", (*globalCounter.lock().unwrap().as_mut().unwrap()));
    }

    (*configData.lock().unwrap().as_mut().unwrap())["build".to_string()] = "debug".to_string();
    (*configData.lock().unwrap().as_mut().unwrap())["target".to_string()] = "rust".to_string();
}

pub fn compute_initial_value() -> Arc<Mutex<Option<i32>>> {

    println!("{}", "Computing initial value during package initialization".to_string());
    return Arc::new(Mutex::new(Some(42 * 2)));
}

/// Another init function that runs after variable initialization
pub fn init() {
    println!("{}", "Fourth init function called".to_string());
    print!("Computed value is: {}\n", (*computedValue.lock().unwrap().as_mut().unwrap()));

    { let mut guard = computedValue.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 10); };
}

pub fn init() {
    println!("{}", "Fifth init function - initializing app config".to_string());
    { let new_val = Config { name: Arc::new(Mutex::new(Some("Go2Rust Transpiler".to_string()))), version: Arc::new(Mutex::new(Some("0.1.0".to_string()))), debug: Arc::new(Mutex::new(Some(true))) }; *appConfig.lock().unwrap() = Some(new_val); };
}

/// Init function that might panic (for testing error handling)
pub fn init() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    println!("{}", "Sixth init function - with potential panic handling".to_string());

    __defer_stack.push(Box::new(move || {
        (Arc::new(Mutex::new(Some(Box::new(move || {
        let mut r = recover();
    if (*r.lock().unwrap()).is_some() {
        print!("Recovered from panic in init: {}\n", (*r.lock().unwrap().as_mut().unwrap()));
    }
    }) as Box<dyn Fn() -> () + Send + Sync>))).lock().unwrap().as_ref().unwrap())();
    }));

    if false {
        panic(Arc::new(Mutex::new(Some("Init function panic!".to_string()))));
    }

    println!("{}", "Sixth init function completed successfully".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}

/// Helper function for init
pub fn setup_logging() {
    println!("{}", "Setting up logging system...".to_string());
}

pub fn init() {
    println!("{}", "Seventh init function - setting up subsystems".to_string());
    setup_logging();

    if (*configData.lock().unwrap().as_mut().unwrap()).len() == 0 {
        println!("{}", "Warning: No configuration data found".to_string());
    } else {
        print!("Configuration loaded with {} entries\n", (*configData.lock().unwrap().as_mut().unwrap()).len());
    }
}

fn main() {
    println!("{}", "\n=== Main function started ===".to_string());

    print!("Global counter: {}\n", (*globalCounter.lock().unwrap().as_mut().unwrap()));
    print!("Initialized flag: {}\n", (*initialized.lock().unwrap().as_mut().unwrap()));
    print!("Computed value: {}\n", (*computedValue.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\nConfiguration data:".to_string());
    for (key, value) in (*configData.lock().unwrap().as_ref().unwrap()).clone() {
        print!("  {}: {}\n", key, (*value.lock().unwrap().as_mut().unwrap()));
    }

    print!("\nApp config: %+v\n", (*appConfig.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Calling functions that were used in init ===".to_string());
    print!("Calling computeInitialValue() again: {}\n", (*compute_initial_value().lock().unwrap().as_mut().unwrap()));
    setup_logging();

    print!("Global counter still: {}\n", (*globalCounter.lock().unwrap().as_mut().unwrap()));

    { let new_val = 100; *globalCounter.lock().unwrap() = Some(new_val); };
    print!("Modified global counter: {}\n", (*globalCounter.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Demonstrating init execution order ===".to_string());
    println!("{}", "1. Package-level variable declarations".to_string());
    println!("{}", "2. Package-level variable initializations (like computedValue)".to_string());
    println!("{}", "3. Init functions in the order they appear in source".to_string());
    println!("{}", "4. Main function".to_string());

    println!("{}", "\n=== Main function completed ===".to_string());
}