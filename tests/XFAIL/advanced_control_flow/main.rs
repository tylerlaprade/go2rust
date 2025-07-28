fn main() {
    println!("{}", "=== Nested loops with labels ===".to_string());
    
    println!("{}", "\n=== Continue with labels ===".to_string());
    
    println!("{}", "\n=== Complex switch with fallthrough ===".to_string());
    let mut num = 1;
    while num <= 5 {
        print!("Number {}: ", num);
        
        println!();
        num += 1;
    }
    println!("{}", "\n=== Nested switch statements ===".to_string());
    let mut category = 1;
    while category <= 2 {
        let mut item = 1;
    while item <= 2 {
        print!("Category {}, Item {}: ", category, item);
        
        item += 1;
    }
        category += 1;
    }
    println!("{}", "\n=== Complex for loop conditions ===".to_string());
    let mut i, let mut j = 0, 10;
    while i < j {
        print!("i={}, j={}, sum={}\n", i, j, i + j);
        
        i, j = i + 1, j - 1;
    }
    println!("{}", "\n=== For loop with complex condition ===".to_string());
    let mut x, let mut y = 1, 1;
    while x * y < 100 && x < 10 {
        print!("x={}, y={}, product={}\n", x, y, x * y);
        
        x += 1;
    }
    println!("{}", "\n=== Goto statements ===".to_string());
    let mut counter = 0;
    
    print!("Counter: {}\n", counter);
    
    println!("{}", "Done with goto".to_string());
    println!("{}", "\n=== Complex if-else chains ===".to_string());
    let mut score = 0;
    while score <= 100 {
        let mut grade = String::new();
        let mut message = String::new();
        
        print!("Score {}: Grade {} - {}\n", score, grade, message);
        score.push_str(&25);
    }
    println!("{}", "\n=== Range with complex break/continue ===".to_string());
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", "Processing numbers:".to_string());
    for (i, num) in numbers.iter().enumerate() {
        
        
        print!("Processing odd number {} (index {})\n", num, i);
    }
    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = vec![, , ];
    for (rowIdx, row) in matrix.iter().enumerate() {
        for (colIdx, cell) in row.iter().enumerate() {
        
        
        print!("[{}][{}]: {} ", rowIdx, colIdx, cell);
    }
        println!();
    }
    println!("{}", "\n=== Select with complex channel operations ===".to_string());
    let mut ch1 = vec![0; 2];
    let mut ch2 = vec![0; 2];
    let mut done = ;
    
    
    
    
    
    ;
    println!("{}", "Channel processing complete".to_string());
    println!("{}", "\n=== Complex error handling flow ===".to_string());
    let mut processData = ;
    let mut testData = vec![, , , , ];
    for (i, data) in testData.iter().enumerate() {
        print!("Testing dataset {}: {}\n", i + 1, data);
        
        print!("  Success: data is valid\n");
    }
}