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
    let (mut i, mut j) = (0, 10);
    while i < j {
        print!("i={}, j={}, sum={}\n", i, j, i + j);
        if i >= 3 {
        
    }
        { i = i + 1; j = j - 1 };
    }
    println!("{}", "\n=== For loop with complex condition ===".to_string());
    let (mut x, mut y) = (1, 1);
    while x * y < 100 && x < 10 {
        print!("x={}, y={}, product={}\n", x, y, x * y);
        if x % 2 == 0 {
        y += 2;
    } else {
        y += 1;
    }
        x += 1;
    }
    println!("{}", "\n=== Goto statements ===".to_string());
    let mut counter = 0;
    
    print!("Counter: {}\n", counter);
    if counter < 3 {
        
    }
    println!("{}", "Done with goto".to_string());
    println!("{}", "\n=== Complex if-else chains ===".to_string());
    let mut score = 0;
    while score <= 100 {
        let mut grade = String::new();
        let mut message = String::new();
        if score >= 90 {
        grade = "A".to_string();
        if score >= 95 {
        message = "Excellent!".to_string();
    } else {
        message = "Great job!".to_string();
    }
    } else if score >= 80 {
        grade = "B".to_string();
        if score >= 85 {
        message = "Good work!".to_string();
    } else {
        message = "Not bad!".to_string();
    }
    } else if score >= 70 {
        grade = "C".to_string();
        message = "Average".to_string();
    } else if score >= 60 {
        grade = "D".to_string();
        message = "Below average".to_string();
    } else {
        grade = "F".to_string();
        message = "Needs improvement".to_string();
    }
        print!("Score {}: Grade {} - {}\n", score, grade, message);
        score += 25;
    }
    println!("{}", "\n=== Range with complex break/continue ===".to_string());
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", "Processing numbers:".to_string());
    for (i, num) in numbers.iter().enumerate() {
        if num % 2 == 0 {
        if num > 6 {
        print!("Stopping at even number {} (index {})\n", num, i);
        
    }
        print!("Skipping even number {} (index {})\n", num, i);
        
    }
        if num == 7 {
        print!("Found lucky number {} at index {}\n", num, i);
        
    }
        print!("Processing odd number {} (index {})\n", num, i);
    }
    println!("{}", "\n=== Nested range loops ===".to_string());
    let mut matrix = vec![, , ];
    for (rowIdx, row) in matrix.iter().enumerate() {
        for (colIdx, cell) in row.iter().enumerate() {
        if cell == "e".to_string() {
        print!("Found center at [{}][{}]: {}\n", rowIdx, colIdx, cell);
        
    }
        if rowIdx == 2 && colIdx == 2 {
        print!("Last cell [{}][{}]: {}\n", rowIdx, colIdx, cell);
        
    }
        print!("[{}][{}]: {} ", rowIdx, colIdx, cell);
    }
        println!();
    }
    println!("{}", "\n=== Select with complex channel operations ===".to_string());
    let mut ch1 = vec![0; 2];
    let mut ch2 = vec![0; 2];
    let mut done = ;
    
    
    
    
    
    <-done;
    println!("{}", "Channel processing complete".to_string());
    println!("{}", "\n=== Complex error handling flow ===".to_string());
    let mut processData = ;
    let mut testData = vec![, , , , ];
    for (i, data) in testData.iter().enumerate() {
        print!("Testing dataset {}: {}\n", i + 1, data);
        let mut err = process_data(data);
    if err.is_some() {
        print!("  Error: {}\n", err);
        
    }
        print!("  Success: data is valid\n");
    }
}