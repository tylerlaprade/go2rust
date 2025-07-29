pub fn divmod(a: i32, b: i32) -> (i32, i32) {

    return (a / b, a % b);
}

pub fn parse_number(s: String) -> (i32, Option<Box<dyn std::error::Error + Send + Sync>>) {

    let (mut num, mut err) = match s.parse::<i32>() { Ok(n) => (n, None), Err(e) => (0, Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)) };
    if err.is_some() {
        return (0, Some(Box::new(format!("failed to parse '{}': {}", s, err)) as Box<dyn std::error::Error + Send + Sync>));
    }
    return (num, None);
}

pub fn get_name_age() -> (String, i32) {

    return ("Alice".to_string(), 30);
}

pub fn calculate(a: i32, b: i32) -> (i32, i32) {
    let mut sum: i32 = 0;
    let mut product: i32 = 0;

    sum = a + b;
    product = a * b;
    return (sum, product);
}

pub fn process_data(data: Vec<i32>) -> (i32, i32, i32) {
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    let mut sum: i32 = 0;

    if data.len() == 0 {
        return (0, 0, 0);
    }
    min = data[0];
    max = data[0];
    sum = 0;
    for (_, val) in data.iter().enumerate() {
        if val < min {
        min = val;
    }
        if val > max {
        max = val;
    }
        sum += val;
    }
    return (min, max, sum);
}

pub fn swap(a: String, b: String) -> (String, String) {

    return (b, a);
}

pub fn get_person_info() -> (String, i32, f64, bool) {
    let mut name: String = String::new();
    let mut age: i32 = 0;
    let mut height: f64 = 0.0;
    let mut married: bool = false;

    return ("Bob".to_string(), 25, 5.9, false);
}

pub fn find_in_slice(slice: Vec<i32>, target: i32) -> (i32, bool) {
    let mut index: i32 = 0;
    let mut found: bool = false;

    for (i, val) in slice.iter().enumerate() {
        if val == target {
        return (i, true);
    }
    }
    return (-1, false);
}

pub fn safe_divide(a: f64, b: f64) -> (f64, Option<Box<dyn std::error::Error + Send + Sync>>) {
    let mut result: f64 = 0.0;
    let mut err: Option<Box<dyn std::error::Error + Send + Sync>> = None;

    if b == 0 {
        return (0, Some(Box::new(format!("division by zero")) as Box<dyn std::error::Error + Send + Sync>));
    }
    return (a / b, None);
}

fn main() {
    println!("{}", "=== Basic multiple returns ===".to_string());
    let (mut quotient, mut remainder) = divmod(17, 5);
    print!("17 / 5 = {} remainder {}\n", quotient, remainder);
    let (mut name, mut age) = get_name_age();
    print!("Name: {}, Age: {}\n", name, age);
    println!("{}", "\n=== Multiple returns with errors ===".to_string());
    let (mut num, mut err) = parse_number("123".to_string());
    if err.is_some() {
        print!("Error: {}\n", err);
    } else {
        print!("Parsed number: {}\n", num);
    }
    (num, err) = parse_number("abc".to_string());
    if err.is_some() {
        print!("Error: {}\n", err);
    } else {
        print!("Parsed number: {}\n", num);
    }
    println!("{}", "\n=== Named return values ===".to_string());
    let (mut s, mut p) = calculate(6, 7);
    print!("Sum: {}, Product: {}\n", s, p);
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (mut min, mut max, mut sum) = process_data(data);
    print!("Data: {}\n", data);
    print!("Min: {}, Max: {}, Sum: {}\n", min, max, sum);
    println!("{}", "\n=== Swapping values ===".to_string());
    let (mut x, mut y) = ("hello".to_string(), "world".to_string());
    print!("Before swap: x={}, y={}\n", x, y);
    (x, y) = swap(x, y);
    print!("After swap: x={}, y={}\n", x, y);
    println!("{}", "\n=== Different types ===".to_string());
    let (mut pName, mut pAge, mut pHeight, mut pMarried) = get_person_info();
    print!("Person: {}, {} years old, {:.1} feet tall, married: {}\n", pName, pAge, pHeight, pMarried);
    println!("{}", "\n=== Finding in slice ===".to_string());
    let mut numbers = vec![10, 20, 30, 40, 50];
    let (mut index, mut found) = find_in_slice(numbers, 30);
    if found {
        print!("Found 30 at index {}\n", index);
    } else {
        println!("{}", "30 not found".to_string());
    }
    (index, found) = find_in_slice(numbers, 99);
    if found {
        print!("Found 99 at index {}\n", index);
    } else {
        println!("{}", "99 not found".to_string());
    }
    println!("{}", "\n=== Safe division ===".to_string());
    let (mut result, mut err) = safe_divide(10.0, 3.0);
    if err.is_some() {
        print!("Error: {}\n", err);
    } else {
        print!("10.0 / 3.0 = {:.2}\n", result);
    }
    (result, err) = safe_divide(10.0, 0.0);
    if err.is_some() {
        print!("Error: {}\n", err);
    } else {
        print!("Result: {:.2}\n", result);
    }
    println!("{}", "\n=== Ignoring return values ===".to_string());
    let (_, mut remainder2) = divmod(23, 7);
    print!("23 mod 7 = {} (quotient ignored)\n", remainder2);
    let (mut name2, _) = get_name_age();
    print!("Name only: {} (age ignored)\n", name2);
    println!("{}", "\n=== Multiple assignment ===".to_string());
    let (mut a, mut b, mut c) = (1, 2, 3);
    print!("a={}, b={}, c={}\n", a, b, c);
    { a = b; b = a };
    print!("After swap: a={}, b={}\n", a, b);
}