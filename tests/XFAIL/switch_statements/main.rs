fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
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
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn basic_switch(day: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    match (*day.lock().unwrap().as_mut().unwrap()) {
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
    match (*x.lock().unwrap().as_mut().unwrap()) * 2 {
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
        true if (*score.lock().unwrap().as_mut().unwrap()) >= 90 => {
            println!("{}", "Grade: A".to_string());
        }
        true if (*score.lock().unwrap().as_mut().unwrap()) >= 80 => {
            println!("{}", "Grade: B".to_string());
        }
        true if (*score.lock().unwrap().as_mut().unwrap()) >= 70 => {
            println!("{}", "Grade: C".to_string());
        }
        true if (*score.lock().unwrap().as_mut().unwrap()) >= 60 => {
            println!("{}", "Grade: D".to_string());
        }
        _ => {
            println!("{}", "Grade: F".to_string());
        }
    }
}

pub fn switch_with_fallthrough(num: std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    match (*num.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "One".to_string());
            // TODO: fallthrough not supported
        }
        2 => {
            println!("{}", "Two or after One".to_string());
            // TODO: fallthrough not supported
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
    // TODO: Unhandled statement type: TypeSwitchStmt
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
    type_switch(true.clone());
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(3.14))));
    type_switch(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3])))))));
}