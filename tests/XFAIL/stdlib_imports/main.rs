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
    println!("{}", "=== Testing multiple stdlib imports ===".to_string());
    println!("{}", "\n--- strings package ---".to_string());
    let mut upper = "hello world".to_string().to_uppercase();
    println!("{} {}", "Upper:".to_string(), (*upper.lock().unwrap().as_mut().unwrap()));
    let mut lower = "HELLO WORLD".to_string().to_lowercase();
    println!("{} {}", "Lower:".to_string(), (*lower.lock().unwrap().as_mut().unwrap()));
    let mut trimmed = "  hello  ".to_string().trim();
    println!("{} {}", "Trimmed:".to_string(), (*trimmed.lock().unwrap().as_mut().unwrap()));
    let mut split = (*strings.lock().unwrap().as_mut().unwrap()).split(std::sync::Arc::new(std::sync::Mutex::new(Some("a,b,c".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(",".to_string()))));
    print!("Split: {}\n", (*split.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- strconv package ---".to_string());
    let mut num = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut str = (*num.lock().unwrap().as_mut().unwrap()).to_string();
    println!("{} {}", "Number as string:".to_string(), (*str.lock().unwrap().as_mut().unwrap()));
    let (mut parsed, mut err) = match "123".to_string().parse::<i32>() { Ok(n) => (std::sync::Arc::new(std::sync::Mutex::new(Some(n))), std::sync::Arc::new(std::sync::Mutex::new(None))), Err(e) => (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))) };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Parse error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Parsed number:".to_string(), (*parsed.lock().unwrap().as_mut().unwrap()));
    }
    let mut floatStr = (*strconv.lock().unwrap().as_mut().unwrap()).format_float(std::sync::Arc::new(std::sync::Mutex::new(Some(3.14159))), std::sync::Arc::new(std::sync::Mutex::new(Some('f'))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(64))));
    println!("{} {}", "Float as string:".to_string(), (*floatStr.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- math package ---".to_string());
    print!("Pi: %.6f\n", (*math.lock().unwrap().as_mut().unwrap()).pi);
    print!("E: %.6f\n", (*math.lock().unwrap().as_mut().unwrap()).e);
    print!("Sqrt(16): {:.2}\n", (*(*math.lock().unwrap().as_mut().unwrap()).sqrt(std::sync::Arc::new(std::sync::Mutex::new(Some(16)))).lock().unwrap().as_mut().unwrap()));
    print!("Pow(2, 8): %.0f\n", (*(*math.lock().unwrap().as_mut().unwrap()).pow(std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(8)))).lock().unwrap().as_mut().unwrap()));
    print!("Max(10, 20): %.0f\n", (*(*math.lock().unwrap().as_mut().unwrap()).max(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20)))).lock().unwrap().as_mut().unwrap()));
    print!("Min(10, 20): %.0f\n", (*(*math.lock().unwrap().as_mut().unwrap()).min(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20)))).lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n--- time package ---".to_string());
    let mut now = (*time.lock().unwrap().as_mut().unwrap()).now();
    println!("{} {}", "Current time:".to_string(), (*(*now.lock().unwrap().as_mut().unwrap()).format(std::sync::Arc::new(std::sync::Mutex::new(Some("2006-01-02 15:04:05".to_string())))).lock().unwrap().as_mut().unwrap()));
    let mut duration = std::sync::Arc::new(std::sync::Mutex::new(Some(5 * (*time.lock().unwrap().as_mut().unwrap()).second)));
    print!("Duration: {}\n", (*duration.lock().unwrap().as_mut().unwrap()));
    println!("{}", "Sleeping for 100ms...".to_string());
    (*time.lock().unwrap().as_mut().unwrap()).sleep(std::sync::Arc::new(std::sync::Mutex::new(Some(100 * (*time.lock().unwrap().as_mut().unwrap()).millisecond))));
    println!("{}", "Done sleeping".to_string());
    println!("{}", "\n--- os package ---".to_string());
    let (mut hostname, mut err) = (*os.lock().unwrap().as_mut().unwrap()).hostname();
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Hostname error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Hostname:".to_string(), (*hostname.lock().unwrap().as_mut().unwrap()));
    }
    let mut path = (*os.lock().unwrap().as_mut().unwrap()).getenv(std::sync::Arc::new(std::sync::Mutex::new(Some("PATH".to_string()))));
    if (*path.lock().unwrap().as_mut().unwrap()) != "".to_string() {
        print!("PATH length: {} characters\n", (*path.lock().unwrap().as_mut().unwrap()).len());
    } else {
        println!("{}", "PATH not found".to_string());
    }
    let (mut wd, mut err) = (*os.lock().unwrap().as_mut().unwrap()).getwd();
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Working directory error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Working directory:".to_string(), (*wd.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "\n--- Combined usage ---".to_string());
    let mut timestamp = (*time.lock().unwrap().as_mut().unwrap()).now().unix();
    let mut timestampStr = (*strconv.lock().unwrap().as_mut().unwrap()).format_int(std::sync::Arc::new(std::sync::Mutex::new(Some((*timestamp.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(10))));
    let mut message = (*strings.lock().unwrap().as_mut().unwrap()).join(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["Timestamp:".to_string(), (*timestampStr.lock().unwrap().as_mut().unwrap())])))))), std::sync::Arc::new(std::sync::Mutex::new(Some(" ".to_string()))));
    println!("{}", (*message.lock().unwrap().as_mut().unwrap()));
    let mut result = (*math.lock().unwrap().as_mut().unwrap()).sqrt(std::sync::Arc::new(std::sync::Mutex::new(Some((*math.lock().unwrap().as_mut().unwrap()).pow(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(2)))) + (*math.lock().unwrap().as_mut().unwrap()).pow(std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))))))));
    let mut resultStr = (*strconv.lock().unwrap().as_mut().unwrap()).format_float(std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some('f'))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(64))));
    print!("Hypotenuse of 3,4 triangle: {}\n", (*resultStr.lock().unwrap().as_mut().unwrap()));
}