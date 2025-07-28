pub fn multiple_returns() -> i32 {
    return 42;
}

pub fn process_slice(slice: Vec<i32>) -> i32 {
    sum = 0;
    count = slice.len();
    for (_, val) in slice.iter().enumerate() {
        sum.push_str(&val);
    }
    return;
}

fn main() {
    println!("{}", "=== Ignoring return values ===".to_string());
    let mut num, let mut _, let mut _ = multiple_returns();
    print!("Only using first return: {}\n", num);
    let mut _, let mut str, let mut _ = multiple_returns();
    print!("Only using middle return: {}\n", str);
    let mut _, let mut _, let mut flag = multiple_returns();
    print!("Only using last return: %t\n", flag);
    println!("{}", "\n=== Ignoring in range loops ===".to_string());
    let mut slice = vec![10, 20, 30, 40, 50];
    println!("{}", "Values only:".to_string());
    for (_, val) in slice.iter().enumerate() {
        print!("{} ", val);
    }
    println!();
    println!("{}", "Indices only:".to_string());
    for (i, _) in slice.iter().enumerate() {
        print!("{} ", i);
    }
    println!();
    println!("{}", "Indices (idiomatic):".to_string());
    for i in 0..slice.len() {
        print!("{} ", i);
    }
    println!();
    println!("{}", "\n=== Ignoring in map iteration ===".to_string());
    let mut ages = std::collections::HashMap::<String, i32>::from([("Alice".to_string(), 25), ("Bob".to_string(), 30), ("Carol".to_string(), 35)]);
    println!("{}", "Keys only:".to_string());
    for (name, _) in ages.iter().enumerate() {
        print!("{} ", name);
    }
    println!();
    println!("{}", "Values only:".to_string());
    for (_, age) in ages.iter().enumerate() {
        print!("{} ", age);
    }
    println!();
    println!("{}", "\n=== Ignoring some return values in assignment ===".to_string());
    let mut sum, let mut _ = process_slice(slice);
    print!("Sum (ignoring count): {}\n", sum);
    let mut _, let mut count = process_slice(slice);
    print!("Count (ignoring sum): {}\n", count);
    println!("{}", "\n=== Blank identifier in declarations ===".to_string());
    _ = "This string is assigned but not used".to_string();
    let mut a, let mut _, let mut c = 1, 2, 3;
    print!("a={}, c={} (middle value ignored)\n", a, c);
    println!("{}", "\n=== Blank identifier with type assertion ===".to_string());
    let mut value = "hello world".to_string();
    
    
    println!("{}", "\n=== Blank identifier with channels ===".to_string());
    let mut ch = vec![0; 3];
    
    
    
    close(ch);
    for  {
        println!("{}", "Received a value (but ignored it)".to_string());
    }
    println!("{}", "\n=== Blank identifier in error handling ===".to_string());
    let mut result, let mut _ = process_slice(vec![1, 2, 3, 4, 5]);
    print!("Result (ignoring potential error): {}\n", result);
    println!("{}", "\n=== Complex example ===".to_string());
    let mut data = vec![, , ];
    let mut total = 0;
    for (_, row) in data.iter().enumerate() {
        for (_, val) in row.iter().enumerate() {
        total.push_str(&val);
    }
    }
    print!("Total of all values: {}\n", total);
}