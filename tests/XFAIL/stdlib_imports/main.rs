use std::error::Error;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

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

fn main() {
        // Testing multiple standard library imports
    println!("{}", "=== Testing multiple stdlib imports ===".to_string());

        // strings package
    println!("{}", "\n--- strings package ---".to_string());
    let mut upper = "hello world".to_string().to_uppercase();
    println!("{} {}", "Upper:".to_string(), (*upper.lock().unwrap().as_mut().unwrap()));

    let mut lower = "HELLO WORLD".to_string().to_lowercase();
    println!("{} {}", "Lower:".to_string(), (*lower.lock().unwrap().as_mut().unwrap()));

    let mut trimmed = "  hello  ".to_string().trim();
    println!("{} {}", "Trimmed:".to_string(), (*trimmed.lock().unwrap().as_mut().unwrap()));

    let mut split = strings.split(Arc::new(Mutex::new(Some("a,b,c".to_string()))), Arc::new(Mutex::new(Some(",".to_string()))));
    print!("Split: {}\n", format_slice(&split));

        // strconv package
    println!("{}", "\n--- strconv package ---".to_string());
    let mut num = Arc::new(Mutex::new(Some(42)));
    let mut str = (*num.lock().unwrap().as_mut().unwrap()).to_string();
    println!("{} {}", "Number as string:".to_string(), (*str.lock().unwrap().as_mut().unwrap()));

    let (mut parsed, mut err) = match "123".to_string().parse::<i32>() { Ok(n) => (Arc::new(Mutex::new(Some(n))), Arc::new(Mutex::new(None))), Err(e) => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::new(e) as Box<dyn Error + Send + Sync>)))) };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Parse error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Parsed number:".to_string(), (*parsed.lock().unwrap().as_mut().unwrap()));
    }

    let mut floatStr = strconv.format_float(Arc::new(Mutex::new(Some(3.14159))), Arc::new(Mutex::new(Some(('f' as i32)))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(64))));
    println!("{} {}", "Float as string:".to_string(), (*floatStr.lock().unwrap().as_mut().unwrap()));

        // math package
    println!("{}", "\n--- math package ---".to_string());
    print!("Pi: %.6f\n", (*(*math.lock().unwrap().as_mut().unwrap())::pi.lock().unwrap().as_ref().unwrap()));
    print!("E: %.6f\n", (*(*math.lock().unwrap().as_mut().unwrap())::e.lock().unwrap().as_ref().unwrap()));
    print!("Sqrt(16): {:.2}\n", (*math.sqrt(Arc::new(Mutex::new(Some(16)))).lock().unwrap().as_ref().unwrap()));
    print!("Pow(2, 8): %.0f\n", (*math.pow(Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(8)))).lock().unwrap().as_ref().unwrap()));
    print!("Max(10, 20): %.0f\n", (*math.max(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20)))).lock().unwrap().as_ref().unwrap()));
    print!("Min(10, 20): %.0f\n", (*math.min(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20)))).lock().unwrap().as_ref().unwrap()));

        // time package
    println!("{}", "\n--- time package ---".to_string());
    let mut now = time.now();
    println!("{} {}", "Current time:".to_string(), (*now.format(Arc::new(Mutex::new(Some("2006-01-02 15:04:05".to_string())))).lock().unwrap().as_ref().unwrap()));

    let mut duration = Arc::new(Mutex::new(Some(5 * (*(*time.lock().unwrap().as_mut().unwrap())::second.lock().unwrap().as_ref().unwrap()))));
    print!("Duration: {}\n", (*duration.lock().unwrap().as_mut().unwrap()));

        // Sleep for a short time
    println!("{}", "Sleeping for 100ms...".to_string());
    time.sleep(Arc::new(Mutex::new(Some(100 * (*(*time.lock().unwrap().as_mut().unwrap())::millisecond.lock().unwrap().as_ref().unwrap())))));
    println!("{}", "Done sleeping".to_string());

        // os package
    println!("{}", "\n--- os package ---".to_string());
    let (mut hostname, mut err) = os.hostname();
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Hostname error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Hostname:".to_string(), (*hostname.lock().unwrap().as_mut().unwrap()));
    }

        // Environment variables
    let mut path = os.getenv(Arc::new(Mutex::new(Some("PATH".to_string()))));
    if (*path.lock().unwrap().as_mut().unwrap()) != "".to_string() {
        print!("PATH length: {} characters\n", (*path.lock().unwrap().as_ref().unwrap()).len());
    } else {
        println!("{}", "PATH not found".to_string());
    }

        // Working directory
    let (mut wd, mut err) = os.getwd();
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Working directory error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Working directory:".to_string(), (*wd.lock().unwrap().as_mut().unwrap()));
    }

        // Combined usage
    println!("{}", "\n--- Combined usage ---".to_string());
    let mut timestamp = time.now().unix();
    let mut timestampStr = strconv.format_int(Arc::new(Mutex::new(Some((*timestamp.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(10))));
    let mut message = strings.join(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec!["Timestamp:".to_string(), (*timestampStr.lock().unwrap().as_mut().unwrap())])))))), Arc::new(Mutex::new(Some(" ".to_string()))));
    println!("{}", (*message.lock().unwrap().as_mut().unwrap()));

        // Mathematical calculation with string formatting
    let mut result = math.sqrt(Arc::new(Mutex::new(Some((*math.pow(Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap()) + (*math.pow(Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap())))));
    let mut resultStr = strconv.format_float(Arc::new(Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(('f' as i32)))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(64))));
    print!("Hypotenuse of 3,4 triangle: {}\n", (*resultStr.lock().unwrap().as_mut().unwrap()));
}