fn main() {
    println!("{}", "=== Testing multiple stdlib imports ===".to_string());
    println!("{}", "\n--- strings package ---".to_string());
    let mut upper = "hello world".to_string().to_uppercase();
    println!("{} {}", "Upper:".to_string(), upper);
    let mut lower = "HELLO WORLD".to_string().to_lowercase();
    println!("{} {}", "Lower:".to_string(), lower);
    let mut trimmed = "  hello  ".to_string().trim();
    println!("{} {}", "Trimmed:".to_string(), trimmed);
    let mut split = strings.split("a,b,c".to_string(), ",".to_string());
    print!("Split: {}\n", split);
    println!("{}", "\n--- strconv package ---".to_string());
    let mut num = 42;
    let mut str = num.to_string();
    println!("{} {}", "Number as string:".to_string(), str);
    let (mut parsed, mut err) = match "123".to_string().parse::<i32>() { Ok(n) => (n, None), Err(e) => (0, Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)) };
    if err.is_some() {
        println!("{} {}", "Parse error:".to_string(), err);
    } else {
        println!("{} {}", "Parsed number:".to_string(), parsed);
    }
    let mut floatStr = strconv.format_float(3.14159, 'f', 2, 64);
    println!("{} {}", "Float as string:".to_string(), floatStr);
    println!("{}", "\n--- math package ---".to_string());
    print!("Pi: %.6f\n", math.pi);
    print!("E: %.6f\n", math.e);
    print!("Sqrt(16): {:.2}\n", math.sqrt(16));
    print!("Pow(2, 8): %.0f\n", math.pow(2, 8));
    print!("Max(10, 20): %.0f\n", math.max(10, 20));
    print!("Min(10, 20): %.0f\n", math.min(10, 20));
    println!("{}", "\n--- time package ---".to_string());
    let mut now = time.now();
    println!("{} {}", "Current time:".to_string(), now.format("2006-01-02 15:04:05".to_string()));
    let mut duration = 5 * time.second;
    print!("Duration: {}\n", duration);
    println!("{}", "Sleeping for 100ms...".to_string());
    time.sleep(100 * time.millisecond);
    println!("{}", "Done sleeping".to_string());
    println!("{}", "\n--- os package ---".to_string());
    let (mut hostname, mut err) = os.hostname();
    if err.is_some() {
        println!("{} {}", "Hostname error:".to_string(), err);
    } else {
        println!("{} {}", "Hostname:".to_string(), hostname);
    }
    let mut path = os.getenv("PATH".to_string());
    if path != "".to_string() {
        print!("PATH length: {} characters\n", path.len());
    } else {
        println!("{}", "PATH not found".to_string());
    }
    let (mut wd, mut err) = os.getwd();
    if err.is_some() {
        println!("{} {}", "Working directory error:".to_string(), err);
    } else {
        println!("{} {}", "Working directory:".to_string(), wd);
    }
    println!("{}", "\n--- Combined usage ---".to_string());
    let mut timestamp = time.now()::unix();
    let mut timestampStr = strconv.format_int(timestamp, 10);
    let mut message = strings.join(vec!["Timestamp:".to_string(), timestampStr], " ".to_string());
    println!("{}", message);
    let mut result = math.sqrt(math.pow(3, 2) + math.pow(4, 2));
    let mut resultStr = strconv.format_float(result, 'f', 2, 64);
    print!("Hypotenuse of 3,4 triangle: {}\n", resultStr);
}