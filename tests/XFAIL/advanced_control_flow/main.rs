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

fn main() {
    println!("{}", "=== Nested loops with labels ===".to_string());

    // TODO: Unhandled statement type: LabeledStmt

    println!("{}", "\n=== Continue with labels ===".to_string());

    // TODO: Unhandled statement type: LabeledStmt

    println!("{}", "\n=== Complex switch with fallthrough ===".to_string());

    let mut num = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*num.lock().unwrap().as_mut().unwrap()) <= 5 {
        print!("Number {}: ", (*num.lock().unwrap().as_mut().unwrap()));
        match (*num.lock().unwrap().as_mut().unwrap()) {
        1 => {
            (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some("One".to_string()))));
            // TODO: fallthrough not supported
        }
        2 => {
            (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(" Two-ish".to_string()))));
        }
        3 => {
            (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some("Three".to_string()))));
        }
        4 | 5 => {
            (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(" Four-or-Five".to_string()))));
        }
        _ => {
            (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(" Other".to_string()))));
        }
    }
        println!();
        { let mut guard = num.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{}", "\n=== Nested switch statements ===".to_string());

    let mut category = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*category.lock().unwrap().as_mut().unwrap()) <= 2 {
        let mut item = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*item.lock().unwrap().as_mut().unwrap()) <= 2 {
        print!("Category {}, Item {}: ", (*category.lock().unwrap().as_mut().unwrap()), (*item.lock().unwrap().as_mut().unwrap()));

        match (*category.lock().unwrap().as_mut().unwrap()) {
        1 => {
            match (*item.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "Electronics - Phone".to_string());
        }
        2 => {
            println!("{}", "Electronics - Laptop".to_string());
        }
        _ => {}
    }
        }
        2 => {
            match (*item.lock().unwrap().as_mut().unwrap()) {
        1 => {
            println!("{}", "Books - Fiction".to_string());
        }
        2 => {
            println!("{}", "Books - Non-fiction".to_string());
        }
        _ => {}
    }
        }
        _ => {}
    }
        { let mut guard = item.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = category.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{}", "\n=== Complex for loop conditions ===".to_string());

    let (mut i, mut j) = (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(10))));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*j.lock().unwrap().as_mut().unwrap()) {
        print!("i={}, j={}, sum={}\n", (*i.lock().unwrap().as_mut().unwrap()), (*j.lock().unwrap().as_mut().unwrap()), (*i.lock().unwrap().as_mut().unwrap()) + (*j.lock().unwrap().as_mut().unwrap()));
        if (*i.lock().unwrap().as_mut().unwrap()) >= 3 {
        break
    }
        { *(*i.lock().unwrap().as_mut().unwrap()).lock().unwrap() = Some((*i.lock().unwrap().as_mut().unwrap()) + 1); *(*j.lock().unwrap().as_mut().unwrap()).lock().unwrap() = Some((*j.lock().unwrap().as_mut().unwrap()) - 1) };
    }

    println!("{}", "\n=== For loop with complex condition ===".to_string());

    let (mut x, mut y) = (std::sync::Arc::new(std::sync::Mutex::new(Some(1))), std::sync::Arc::new(std::sync::Mutex::new(Some(1))));
    while (*x.lock().unwrap().as_mut().unwrap()) * (*y.lock().unwrap().as_mut().unwrap()) < 100 && (*x.lock().unwrap().as_mut().unwrap()) < 10 {
        print!("x={}, y={}, product={}\n", (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()), (*x.lock().unwrap().as_mut().unwrap()) * (*y.lock().unwrap().as_mut().unwrap()));
        if (*x.lock().unwrap().as_mut().unwrap()) % 2 == 0 {
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 2); };
    } else {
        { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
    }
        { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    println!("{}", "\n=== Goto statements ===".to_string());

    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));

    // TODO: Unhandled statement type: LabeledStmt
    print!("Counter: {}\n", (*counter.lock().unwrap().as_mut().unwrap()));

    if (*counter.lock().unwrap().as_mut().unwrap()) < 3 {
        // TODO: goto not supported
    }

    println!("{}", "Done with goto".to_string());

    println!("{}", "\n=== Complex if-else chains ===".to_string());

    let mut score = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*score.lock().unwrap().as_mut().unwrap()) <= 100 {
        let mut grade: std::sync::Arc<std::sync::Mutex<Option<String>>> = String::new();
        let mut message: std::sync::Arc<std::sync::Mutex<Option<String>>> = String::new();

        if (*score.lock().unwrap().as_mut().unwrap()) >= 90 {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
        if (*score.lock().unwrap().as_mut().unwrap()) >= 95 {
        { let new_val = "Excellent!".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "Great job!".to_string(); *message.lock().unwrap() = Some(new_val); };
    }
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 80 {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
        if (*score.lock().unwrap().as_mut().unwrap()) >= 85 {
        { let new_val = "Good work!".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "Not bad!".to_string(); *message.lock().unwrap() = Some(new_val); };
    }
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 70 {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Average".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 60 {
        { let new_val = "D".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Below average".to_string(); *message.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
        { let new_val = "Needs improvement".to_string(); *message.lock().unwrap() = Some(new_val); };
    }

        print!("Score {}: Grade {} - {}\n", (*score.lock().unwrap().as_mut().unwrap()), (*grade.lock().unwrap().as_mut().unwrap()), (*message.lock().unwrap().as_mut().unwrap()));
        { let mut guard = score.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 25); };
    }

    println!("{}", "\n=== Range with complex break/continue ===".to_string());

    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Processing numbers:".to_string());
    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        if num % 2 == 0 {
        if num > 6 {
        print!("Stopping at even number {} (index {})\n", num, i);
        break
    }
        print!("Skipping even number {} (index {})\n", num, i);
        continue
    }
        if num == 7 {
        print!("Found lucky number {} at index {}\n", num, i);
        continue
    }
        print!("Processing odd number {} (index {})\n", num, i);
    }

    println!("{}", "\n=== Nested range loops ===".to_string());

    let mut matrix = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));

    for (rowIdx, row) in (*matrix.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        for (colIdx, cell) in row.iter().enumerate() {
        if cell == "e".to_string() {
        print!("Found center at [{}][{}]: {}\n", rowIdx, colIdx, cell);
        continue
    }
        if rowIdx == 2 && colIdx == 2 {
        print!("Last cell [{}][{}]: {}\n", rowIdx, colIdx, cell);
        break
    }
        print!("[{}][{}]: {} ", rowIdx, colIdx, cell);
    }
        println!();
    }

    println!("{}", "\n=== Select with complex channel operations ===".to_string());

    let mut ch1 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 2])));
    let mut ch2 = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![std::sync::Arc::new(std::sync::Mutex::new(Some(0))); 2])));
    let mut done = ;

    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt

    // TODO: Unhandled statement type: GoStmt

    <-(*done.lock().unwrap().as_mut().unwrap());
    println!("{}", "Channel processing complete".to_string());

    println!("{}", "\n=== Complex error handling flow ===".to_string());

    let mut processData = std::sync::Arc::new(std::sync::Mutex::new(Some()));

    let mut testData = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , , , ])));

    for (i, data) in (*testData.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Testing dataset {}: {}\n", i + 1, data);
        let mut err = process_data(std::sync::Arc::new(std::sync::Mutex::new(Some(data))));
    if (*err.lock().unwrap()).is_some() {
        print!("  Error: {}\n", (*err.lock().unwrap().as_mut().unwrap()));
        continue
    }
        print!("  Success: data is valid\n");
    }
}