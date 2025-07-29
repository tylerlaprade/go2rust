fn main() {
    println!("{}", "=== Range over slice ===".to_string());
    let mut numbers = vec![10, 20, 30, 40, 50];
    for (i, num) in numbers.iter().enumerate() {
        print!("Index {}: {}\n", i, num);
    }
    println!("{}", "Values only:".to_string());
    for (_, num) in numbers.iter().enumerate() {
        print!("{} ", num);
    }
    println!();
    println!("{}", "Indices only:".to_string());
    for i in 0..numbers.len() {
        print!("{} ", i);
    }
    println!();
    println!("{}", "\n=== Range over array ===".to_string());
    let mut arr = ["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()];
    for (i, fruit) in arr.iter().enumerate() {
        print!("{}: {}\n", i, fruit);
    }
    println!("{}", "\n=== Range over string ===".to_string());
    let mut text = "Hello, 世界".to_string();
    for (i, char) in text.iter().enumerate() {
        print!("Byte {}: %c (Unicode: %U)\n", i, char, char);
    }
    println!("{}", "\n=== Range over map ===".to_string());
    let mut ages = std::collections::HashMap::<String, i32>::from([("Alice".to_string(), 25), ("Bob".to_string(), 30), ("Charlie".to_string(), 35)]);
    for (name, age) in &ages {
        print!("{} is {} years old\n", name, age);
    }
    println!("{}", "Keys only:".to_string());
    for (name, _) in &ages {
        print!("{} ", name);
    }
    println!();
    println!("{}", "\n=== Range over channel ===".to_string());
    let mut ch = vec![0; 5];
    let mut i = 1;
    while i <= 5 {
        
        i += 1;
    }
    close(ch);
    for value in 0..ch.len() {
        print!("Received: {}\n", value);
    }
    println!("{}", "\n=== Range with break/continue ===".to_string());
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", "Even numbers only (with continue):".to_string());
    for (_, num) in data.iter().enumerate() {
        if num % 2 != 0 {
        
    }
        print!("{} ", num);
    }
    println!();
    println!("{}", "Numbers until 6 (with break):".to_string());
    for (_, num) in data.iter().enumerate() {
        if num > 6 {
        
    }
        print!("{} ", num);
    }
    println!();
    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = vec![, , ];
    for (i, row) in matrix.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
        print!("matrix[{}][{}] = {}\n", i, j, val);
    }
    }
    println!("{}", "\n=== Range over empty collections ===".to_string());
    let mut emptySlice: Vec<i32> = Default::default();
    let mut emptyMap;
    println!("{}", "Empty slice:".to_string());
    for (i, v) in emptySlice.iter().enumerate() {
        print!("This won't print: {}, {}\n", i, v);
    }
    println!("{}", "Empty slice range completed".to_string());
    println!("{}", "Empty map:".to_string());
    for (k, v) in &emptyMap {
        print!("This won't print: {}, {}\n", k, v);
    }
    println!("{}", "Empty map range completed".to_string());
}