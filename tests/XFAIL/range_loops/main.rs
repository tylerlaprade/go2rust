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
    println!("{}", "=== Range over slice ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30, 40, 50])));

    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Index {}: {}\n", i, num);
    }

    println!("{}", "Values only:".to_string());
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        print!("{} ", num);
    }
    println!();

    println!("{}", "Indices only:".to_string());
    for i in 0..(*numbers.lock().unwrap().as_mut().unwrap()).len() {
        print!("{} ", i);
    }
    println!();

    println!("{}", "\n=== Range over array ===".to_string());
    let mut arr = std::sync::Arc::new(std::sync::Mutex::new(Some(["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()])));
    for (i, fruit) in (*arr.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("{}: {}\n", i, fruit);
    }

    println!("{}", "\n=== Range over string ===".to_string());
    let mut text = std::sync::Arc::new(std::sync::Mutex::new(Some("Hello, 世界".to_string())));
    for (i, char) in (*text.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Byte {}: {} (Unicode: {:?})\n", i, char, char);
    }

    println!("{}", "\n=== Range over map ===".to_string());
    let mut ages = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::from([("Alice".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(25)))), ("Bob".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(30)))), ("Charlie".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(35))))]))));

    for (name, age) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} is {} years old\n", name, (*age.lock().unwrap().as_mut().unwrap()));
    }

    println!("{}", "Keys only:".to_string());
    for (name, _) in (*ages.lock().unwrap().as_ref().unwrap()).clone() {
        print!("{} ", name);
    }
    println!();

    println!("{}", "\n=== Range over channel ===".to_string());
    let mut ch = ;

    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    while (*i.lock().unwrap().as_mut().unwrap()) <= 5 {
        // TODO: Unhandled statement type: SendStmt
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    close(ch.clone());

    for value in 0..(*ch.lock().unwrap().as_mut().unwrap()).len() {
        print!("Received: {}\n", value);
    }

    println!("{}", "\n=== Range with break/continue ===".to_string());
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    println!("{}", "Even numbers only (with continue):".to_string());
    for num in &(*data.lock().unwrap().as_mut().unwrap()) {
        if num % 2 != 0 {
        continue
    }
        print!("{} ", num);
    }
    println!();

    println!("{}", "Numbers until 6 (with break):".to_string());
    for num in &(*data.lock().unwrap().as_mut().unwrap()) {
        if num > 6 {
        break
    }
        print!("{} ", num);
    }
    println!();

    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));

    for (i, row) in (*matrix.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
        print!("matrix[{}][{}] = {}\n", i, j, val);
    }
    }

    println!("{}", "\n=== Range over empty collections ===".to_string());
    let mut emptySlice: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(Default::default())));
    let mut emptyMap: std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<String, i32>>>>;

    println!("{}", "Empty slice:".to_string());
    for (i, v) in (*emptySlice.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("This won't print: {}, {}\n", i, v);
    }
    println!("{}", "Empty slice range completed".to_string());

    println!("{}", "Empty map:".to_string());
    for (k, v) in (*emptyMap.lock().unwrap().as_ref().unwrap()).clone() {
        print!("This won't print: {}, {}\n", k, (*v.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "Empty map range completed".to_string());
}