pub fn divmod(a: i32, b: i32) -> i32 {
    return a / b;
}

pub fn parse_number(s: String) -> i32 {
    let mut num, let mut err = strconv.atoi(s);
    
    return num;
}

pub fn get_name_age() -> String {
    return "Alice".to_string();
}

pub fn calculate(a: i32, b: i32) -> i32 {
    sum = a + b;
    product = a * b;
    return;
}

pub fn process_data(data: Vec<i32>) -> i32 {
    
    min = data[0];
    max = data[0];
    sum = 0;
    for (_, val) in data.iter().enumerate() {
        
        
        sum.push_str(&val);
    }
    return;
}

pub fn swap(a: String, b: String) -> String {
    return b;
}

pub fn get_person_info() -> String {
    return "Bob".to_string();
}

pub fn find_in_slice(slice: Vec<i32>, target: i32) -> i32 {
    for (i, val) in slice.iter().enumerate() {
        
    }
    return ;
}

pub fn safe_divide(a: f64, b: f64) -> f64 {
    
    return a / b;
}

fn main() {
    println!("{}", "=== Basic multiple returns ===".to_string());
    let mut quotient, let mut remainder = divmod(17, 5);
    print!("17 / 5 = {} remainder {}\n", quotient, remainder);
    let mut name, let mut age = get_name_age();
    print!("Name: {}, Age: {}\n", name, age);
    println!("{}", "\n=== Multiple returns with errors ===".to_string());
    let mut num, let mut err = parse_number("123".to_string());
    
    num, err = parse_number("abc".to_string());
    
    println!("{}", "\n=== Named return values ===".to_string());
    let mut s, let mut p = calculate(6, 7);
    print!("Sum: {}, Product: {}\n", s, p);
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let mut min, let mut max, let mut sum = process_data(data);
    print!("Data: {}\n", data);
    print!("Min: {}, Max: {}, Sum: {}\n", min, max, sum);
    println!("{}", "\n=== Swapping values ===".to_string());
    let mut x, let mut y = "hello".to_string(), "world".to_string();
    print!("Before swap: x={}, y={}\n", x, y);
    x, y = swap(x, y);
    print!("After swap: x={}, y={}\n", x, y);
    println!("{}", "\n=== Different types ===".to_string());
    let mut pName, let mut pAge, let mut pHeight, let mut pMarried = get_person_info();
    print!("Person: {}, {} years old, %.1f feet tall, married: %t\n", pName, pAge, pHeight, pMarried);
    println!("{}", "\n=== Finding in slice ===".to_string());
    let mut numbers = vec![10, 20, 30, 40, 50];
    let mut index, let mut found = find_in_slice(numbers, 30);
    
    index, found = find_in_slice(numbers, 99);
    
    println!("{}", "\n=== Safe division ===".to_string());
    let mut result, let mut err = safe_divide(10.0, 3.0);
    
    result, err = safe_divide(10.0, 0.0);
    
    println!("{}", "\n=== Ignoring return values ===".to_string());
    let mut _, let mut remainder2 = divmod(23, 7);
    print!("23 mod 7 = {} (quotient ignored)\n", remainder2);
    let mut name2, let mut _ = get_name_age();
    print!("Name only: {} (age ignored)\n", name2);
    println!("{}", "\n=== Multiple assignment ===".to_string());
    let mut a, let mut b, let mut c = 1, 2, 3;
    print!("a={}, b={}, c={}\n", a, b, c);
    a, b = b, a;
    print!("After swap: a={}, b={}\n", a, b);
}