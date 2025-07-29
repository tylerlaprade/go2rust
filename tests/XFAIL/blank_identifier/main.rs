pub fn multiple_returns() -> (i32, String, bool) {

    return (42, "hello".to_string(), true);
}

pub fn process_slice(slice: Vec<i32>) -> (i32, i32) {
    let mut sum: i32 = 0;
    let mut count: i32 = 0;

    sum = 0;
    count = slice.len();
    for (_, val) in slice.iter().enumerate() {
        sum += val;
    }
    return (sum, count);
}

fn main() {
    println!("{}", "=== Ignoring return values ===".to_string());
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", num);
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", str);
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", flag);
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
    let (mut sum, _) = process_slice(slice);
    print!("Sum (ignoring count): {}\n", sum);
    let (_, mut count) = process_slice(slice);
    print!("Count (ignoring sum): {}\n", count);
    println!("{}", "\n=== Blank identifier in declarations ===".to_string());
    _ = "This string is assigned but not used".to_string();
    let (mut a, _, mut c) = (1, 2, 3);
    print!("a={}, c={} (middle value ignored)\n", a, c);
    println!("{}", "\n=== Blank identifier with type assertion ===".to_string());
    let mut value = "hello world".to_string();
    let (_, mut ok) = ;
    if ok {
        println!("{}", "Value is a string (but we ignored the actual value)".to_string());
    }
    let (_, mut ok) = ;
    if ok {
        println!("{}", "Value is an int".to_string());
    } else {
        println!("{}", "Value is not an int".to_string());
    }
    println!("{}", "\n=== Blank identifier with channels ===".to_string());
    let mut ch = vec![0; 3];
    
    
    
    close(ch);
    for  {
        println!("{}", "Received a value (but ignored it)".to_string());
    }
    println!("{}", "\n=== Blank identifier in error handling ===".to_string());
    let (mut result, _) = process_slice(vec![1, 2, 3, 4, 5]);
    print!("Result (ignoring potential error): {}\n", result);
    println!("{}", "\n=== Complex example ===".to_string());
    let mut data = vec![, , ];
    let mut total = 0;
    for (_, row) in data.iter().enumerate() {
        for (_, val) in row.iter().enumerate() {
        total += val;
    }
    }
    print!("Total of all values: {}\n", total);
}