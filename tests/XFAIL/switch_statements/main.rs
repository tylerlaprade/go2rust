pub fn basic_switch(day: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    match (*day.lock().unwrap().as_ref().unwrap()) {
        1 => {
            println!("{}", "Monday".to_string());
        }
        2 => {
            println!("{}", "Tuesday".to_string());
        }
        3 => {
            println!("{}", "Wednesday".to_string());
        }
        4 => {
            println!("{}", "Thursday".to_string());
        }
        5 => {
            println!("{}", "Friday".to_string());
        }
        6 | 7 => {
            println!("{}", "Weekend".to_string());
        }
        _ => {
            println!("{}", "Invalid day".to_string());
        }
    }
}

pub fn switch_with_expression() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(10)));
    match (*x.lock().unwrap().as_ref().unwrap()) * 2 {
        20 => {
            println!("{}", "x * 2 equals 20".to_string());
        }
        30 => {
            println!("{}", "x * 2 equals 30".to_string());
        }
        _ => {
            println!("{}", "x * 2 is something else".to_string());
        }
    }
}

pub fn switch_without_expression() {
    let mut score = std::sync::Arc::new(std::sync::Mutex::new(Some(85)));
    match true {
        true if (*score.lock().unwrap().as_ref().unwrap()) >= 90 => {
            println!("{}", "Grade: A".to_string());
        }
        true if (*score.lock().unwrap().as_ref().unwrap()) >= 80 => {
            println!("{}", "Grade: B".to_string());
        }
        true if (*score.lock().unwrap().as_ref().unwrap()) >= 70 => {
            println!("{}", "Grade: C".to_string());
        }
        true if (*score.lock().unwrap().as_ref().unwrap()) >= 60 => {
            println!("{}", "Grade: D".to_string());
        }
        _ => {
            println!("{}", "Grade: F".to_string());
        }
    }
}

pub fn switch_with_fallthrough(num: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    match (*num.lock().unwrap().as_ref().unwrap()) {
        1 => {
            println!("{}", "One".to_string());
            
        }
        2 => {
            println!("{}", "Two or after One".to_string());
            
        }
        3 => {
            println!("{}", "Three or after Two or after One".to_string());
        }
        _ => {
            println!("{}", "Other number".to_string());
        }
    }
}

pub fn type_switch(value: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>>) {
    
}

fn main() {
    println!("{}", "=== Basic switch ===".to_string());
    basic_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(1))));
    basic_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(6))));
    basic_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(10))));
    println!("{}", "\n=== Switch with expression ===".to_string());
    switch_with_expression();
    println!("{}", "\n=== Switch without expression ===".to_string());
    switch_without_expression();
    println!("{}", "\n=== Switch with fallthrough ===".to_string());
    switch_with_fallthrough(std::sync::Arc::new(std::sync::Mutex::new(Some(1))));
    println!("{}", "---".to_string());
    switch_with_fallthrough(std::sync::Arc::new(std::sync::Mutex::new(Some(4))));
    println!("{}", "\n=== Type switch ===".to_string());
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(42))));
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string()))));
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(true))));
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(3.14))));
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3]))));
}