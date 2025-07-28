fn main() {
    println!("{}", "=== Complex arithmetic expressions ===".to_string());
    let mut a, let mut b, let mut c = 10, 20, 30;
    let mut result1 =  * c -  / ;
    print!("(a + b) * c - (a * b) / (c - a) = {}\n", result1);
    let mut result2 = a + b * c /  + c % b;
    print!("a + b * c / (a - 5) + c %% b = {}\n", result2);
    println!("{}", "\n=== Complex boolean expressions ===".to_string());
    let mut x, let mut y, let mut z = 5, 10, 15;
    let mut bool1 =  &&  || ;
    print!("(x < y) && (y < z) || (x == 5 && z > 10) = %t\n", bool1);
    let mut bool2 =  &&  || ;
    print!("!(x > y) && (z-y == x) || (x*2 == y) = %t\n", bool2);
    println!("{}", "\n=== Complex bitwise expressions ===".to_string());
    let mut bits1, let mut bits2 = 0b1010, 0b1100;
    let mut bitwiseResult =  |  << 1;
    print!("(bits1 & bits2) | (bits1 ^ bits2) << 1 = %b ({})\n", bitwiseResult, bitwiseResult);
    println!("{}", "\n=== Function calls in expressions ===".to_string());
    let mut getValue = ;
    let mut getMultiplier = ;
    let mut complexResult = get_value(a) + get_value(b) * get_multiplier() - get_value(c) / 2;
    print!("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = {}\n", complexResult);
    println!("{}", "\n=== Array/slice expressions ===".to_string());
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut idx1, let mut idx2 = 2, 7;
    let mut sliceResult = numbers[idx1..idx2].to_vec()[1] + numbers[numbers.len() - 1] - numbers[0];
    print!("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = {}\n", sliceResult);
    println!("{}", "\n=== Map expressions ===".to_string());
    let mut data = std::collections::HashMap::<String, i32>::from([("alpha".to_string(), 10), ("beta".to_string(), 20), ("gamma".to_string(), 30)]);
    let mut mapResult = data["alpha".to_string()] + data["beta".to_string()] * 2 - data["gamma".to_string()] / 3;
    print!("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = {}\n", mapResult);
    println!("{}", "\n=== Struct field expressions ===".to_string());
    
    let mut p1 = Point { x: 3, y: 4 };
    let mut p2 = Point { x: 6, y: 8 };
    let mut distanceSquared =  *  +  * ;
    print!("Distance squared between points: {}\n", distanceSquared);
    println!("{}", "\n=== Pointer expressions ===".to_string());
    let mut val = 42;
    let mut ptr = ;
    let mut ptrResult =  +  - ;
    print!("*ptr + (*ptr * 2) - (*ptr / 2) = {}\n", ptrResult);
    println!("{}", "\n=== Type assertion expressions ===".to_string());
    let mut iface = 100;
    
    println!("{}", "\n=== Channel expressions ===".to_string());
    let mut ch = vec![0; 3];
    
    
    
    let mut chanResult =  +  * 2 -  / 2;
    print!("Channel expression result: {}\n", chanResult);
    println!("{}", "\n=== Nested function calls ===".to_string());
    let mut add = ;
    let mut multiply = ;
    let mut subtract = ;
    let mut nestedResult = add(multiply(3, 4), subtract(20, multiply(2, 5)));
    print!("add(multiply(3, 4), subtract(20, multiply(2, 5))) = {}\n", nestedResult);
    println!("{}", "\n=== Complex conditional expressions ===".to_string());
    let mut score = 85;
    let mut grade = String::new();
    
    print!("Grade for score {}: {}\n", score, grade);
    println!("{}", "\n=== Complex assignment expressions ===".to_string());
    let mut counter = 0;
    counter.push_str(& -  + );
    print!("Complex assignment result: {}\n", counter);
    let mut sum, let mut product = a + b + c, a * b * c;
    print!("Sum: {}, Product: {}\n", sum, product);
    println!("{}", "\n=== Range expressions ===".to_string());
    let mut total = 0;
    for (i, val) in numbers[..5].to_vec().iter().enumerate() {
        total.push_str(&i * val + );
    }
    print!("Complex range calculation: {}\n", total);
}