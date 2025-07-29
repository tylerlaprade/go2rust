fn main() {
    println!("{}", "=== Complex arithmetic expressions ===".to_string());
    let (mut (*a.lock().unwrap().as_ref().unwrap()), mut (*b.lock().unwrap().as_ref().unwrap()), mut (*c.lock().unwrap().as_ref().unwrap())) = (10, 20, 30);
    let mut result1 = std::sync::Arc::new(std::sync::Mutex::new(Some( * (*c.lock().unwrap().as_ref().unwrap()) -  / )));
    print!("(a + b) * c - (a * b) / (c - a) = {}\n", (*result1.lock().unwrap().as_ref().unwrap()));
    let mut result2 = std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) + (*b.lock().unwrap().as_ref().unwrap()) * (*c.lock().unwrap().as_ref().unwrap()) /  + (*c.lock().unwrap().as_ref().unwrap()) % (*b.lock().unwrap().as_ref().unwrap()))));
    print!("a + b * c / (a - 5) + c %% b = {}\n", (*result2.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Complex boolean expressions ===".to_string());
    let (mut (*x.lock().unwrap().as_ref().unwrap()), mut (*y.lock().unwrap().as_ref().unwrap()), mut (*z.lock().unwrap().as_ref().unwrap())) = (5, 10, 15);
    let mut bool1 = std::sync::Arc::new(std::sync::Mutex::new(Some( &&  || )));
    print!("(x < y) && (y < z) || (x == 5 && z > 10) = {}\n", (*bool1.lock().unwrap().as_ref().unwrap()));
    let mut bool2 = std::sync::Arc::new(std::sync::Mutex::new(Some(! &&  || )));
    print!("!(x > y) && (z-y == x) || (x*2 == y) = {}\n", (*bool2.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Complex bitwise expressions ===".to_string());
    let (mut (*bits1.lock().unwrap().as_ref().unwrap()), mut (*bits2.lock().unwrap().as_ref().unwrap())) = (0b1010, 0b1100);
    let mut bitwiseResult = std::sync::Arc::new(std::sync::Mutex::new(Some( |  << 1)));
    print!("(bits1 & bits2) | (bits1 ^ bits2) << 1 = %b ({})\n", (*bitwiseResult.lock().unwrap().as_ref().unwrap()), (*bitwiseResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Function calls in expressions ===".to_string());
    let mut getValue = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut getMultiplier = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut complexResult = std::sync::Arc::new(std::sync::Mutex::new(Some(get_value(std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()))))) + get_value(std::sync::Arc::new(std::sync::Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()))))) * get_multiplier() - get_value(std::sync::Arc::new(std::sync::Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()))))) / 2)));
    print!("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = {}\n", (*complexResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Array/slice expressions ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
    let (mut (*idx1.lock().unwrap().as_ref().unwrap()), mut (*idx2.lock().unwrap().as_ref().unwrap())) = (2, 7);
    let mut sliceResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())[(*idx1.lock().unwrap().as_ref().unwrap())..(*idx2.lock().unwrap().as_ref().unwrap())].to_vec()[1] + (*numbers.lock().unwrap().as_ref().unwrap())[(*numbers.lock().unwrap().as_ref().unwrap()).len() - 1] - (*numbers.lock().unwrap().as_ref().unwrap())[0])));
    print!("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = {}\n", (*sliceResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Map expressions ===".to_string());
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::from([("alpha".to_string(), 10), ("beta".to_string(), 20), ("gamma".to_string(), 30)]))));
    let mut mapResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*data.lock().unwrap().as_ref().unwrap())["alpha".to_string()] + (*data.lock().unwrap().as_ref().unwrap())["beta".to_string()] * 2 - (*data.lock().unwrap().as_ref().unwrap())["gamma".to_string()] / 3)));
    print!("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = {}\n", (*mapResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Struct field expressions ===".to_string());
    
    let mut p1 = std::sync::Arc::new(std::sync::Mutex::new(Some(Point { x: 3, y: 4 })));
    let mut p2 = std::sync::Arc::new(std::sync::Mutex::new(Some(Point { x: 6, y: 8 })));
    let mut distanceSquared = std::sync::Arc::new(std::sync::Mutex::new(Some( *  +  * )));
    print!("Distance squared between points: {}\n", (*distanceSquared.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Pointer expressions ===".to_string());
    let mut val = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut ptr = val.clone();
    let mut ptrResult = std::sync::Arc::new(std::sync::Mutex::new(Some(*(*ptr.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap() +  - )));
    print!("*ptr + (*ptr * 2) - (*ptr / 2) = {}\n", (*ptrResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Type assertion expressions ===".to_string());
    let mut iface = 100;
    let (mut (*intVal.lock().unwrap().as_ref().unwrap()), mut (*ok.lock().unwrap().as_ref().unwrap())) = match (*iface.lock().unwrap().as_ref().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_ref().unwrap()) {
        let mut assertResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*intVal.lock().unwrap().as_ref().unwrap()) * 2 +  * 3)));
        print!("Type assertion result: {}\n", (*assertResult.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "\n=== Channel expressions ===".to_string());
    let mut ch = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; 3])));
    
    
    
    let mut chanResult = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*ch.lock().unwrap().as_ref().unwrap()) + <-(*ch.lock().unwrap().as_ref().unwrap()) * 2 - <-(*ch.lock().unwrap().as_ref().unwrap()) / 2)));
    print!("Channel expression result: {}\n", (*chanResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Nested function calls ===".to_string());
    let mut add = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut multiply = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut subtract = std::sync::Arc::new(std::sync::Mutex::new(Some()));
    let mut nestedResult = std::sync::Arc::new(std::sync::Mutex::new(Some(add(std::sync::Arc::new(std::sync::Mutex::new(Some(multiply(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4))))))), std::sync::Arc::new(std::sync::Mutex::new(Some(subtract(std::sync::Arc::new(std::sync::Mutex::new(Some(20))), std::sync::Arc::new(std::sync::Mutex::new(Some(multiply(std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))))))))))))));
    print!("add(multiply(3, 4), subtract(20, multiply(2, 5))) = {}\n", (*nestedResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Complex conditional expressions ===".to_string());
    let mut score = std::sync::Arc::new(std::sync::Mutex::new(Some(85)));
    let mut grade = String::new();
    if (*score.lock().unwrap().as_ref().unwrap()) >= 90 {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_ref().unwrap()) >= 80 {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_ref().unwrap()) >= 70 {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
    }
    print!("Grade for score {}: {}\n", (*score.lock().unwrap().as_ref().unwrap()), (*grade.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Complex assignment expressions ===".to_string());
    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    (*counter.lock().unwrap().as_ref().unwrap()) +=  -  + ;
    print!("Complex assignment result: {}\n", (*counter.lock().unwrap().as_ref().unwrap()));
    let (mut (*sum.lock().unwrap().as_ref().unwrap()), mut (*product.lock().unwrap().as_ref().unwrap())) = ((*a.lock().unwrap().as_ref().unwrap()) + (*b.lock().unwrap().as_ref().unwrap()) + (*c.lock().unwrap().as_ref().unwrap()), (*a.lock().unwrap().as_ref().unwrap()) * (*b.lock().unwrap().as_ref().unwrap()) * (*c.lock().unwrap().as_ref().unwrap()));
    print!("Sum: {}, Product: {}\n", (*sum.lock().unwrap().as_ref().unwrap()), (*product.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Range expressions ===".to_string());
    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    for (i, val) in (*numbers.lock().unwrap().as_ref().unwrap())[..5].to_vec().iter().enumerate() {
        (*total.lock().unwrap().as_ref().unwrap()) += (*i.lock().unwrap().as_ref().unwrap()) * (*val.lock().unwrap().as_ref().unwrap()) + ;
    }
    print!("Complex range calculation: {}\n", (*total.lock().unwrap().as_ref().unwrap()));
}