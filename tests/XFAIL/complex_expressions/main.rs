fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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
    println!("{}", "=== Complex arithmetic expressions ===".to_string());

    let (mut a, mut b, mut c) = (std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20))), std::sync::Arc::new(std::sync::Mutex::new(Some(30))));

    let mut result1 = std::sync::Arc::new(std::sync::Mutex::new(Some(((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap())) * (*c.lock().unwrap().as_mut().unwrap()) - ((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap())) / ((*c.lock().unwrap().as_mut().unwrap()) - (*a.lock().unwrap().as_mut().unwrap())))));
    print!("(a + b) * c - (a * b) / (c - a) = {}\n", (*result1.lock().unwrap().as_mut().unwrap()));

    let mut result2 = std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()) * (*c.lock().unwrap().as_mut().unwrap()) / ((*a.lock().unwrap().as_mut().unwrap()) - 5) + (*c.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap()))));
    print!("a + b * c / (a - 5) + c %% b = {}\n", (*result2.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Complex boolean expressions ===".to_string());

    let (mut x, mut y, mut z) = (std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(15))));

    let mut bool1 = std::sync::Arc::new(std::sync::Mutex::new(Some(((*x.lock().unwrap().as_mut().unwrap()) < (*y.lock().unwrap().as_mut().unwrap())) && ((*y.lock().unwrap().as_mut().unwrap()) < (*z.lock().unwrap().as_mut().unwrap())) || ((*x.lock().unwrap().as_mut().unwrap()) == 5 && (*z.lock().unwrap().as_mut().unwrap()) > 10))));
    print!("(x < y) && (y < z) || (x == 5 && z > 10) = {}\n", (*bool1.lock().unwrap().as_mut().unwrap()));

    let mut bool2 = std::sync::Arc::new(std::sync::Mutex::new(Some(!((*x.lock().unwrap().as_mut().unwrap()) > (*y.lock().unwrap().as_mut().unwrap())) && ((*z.lock().unwrap().as_mut().unwrap()) - (*y.lock().unwrap().as_mut().unwrap()) == (*x.lock().unwrap().as_mut().unwrap())) || ((*x.lock().unwrap().as_mut().unwrap()) * 2 == (*y.lock().unwrap().as_mut().unwrap())))));
    print!("!(x > y) && (z-y == x) || (x*2 == y) = {}\n", (*bool2.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Complex bitwise expressions ===".to_string());

    let (mut bits1, mut bits2) = (std::sync::Arc::new(std::sync::Mutex::new(Some(0b1010))), std::sync::Arc::new(std::sync::Mutex::new(Some(0b1100))));

    let mut bitwiseResult = std::sync::Arc::new(std::sync::Mutex::new(Some(((*bits1.lock().unwrap().as_mut().unwrap()) & (*bits2.lock().unwrap().as_mut().unwrap())) | ((*bits1.lock().unwrap().as_mut().unwrap()) ^ (*bits2.lock().unwrap().as_mut().unwrap())) << 1)));
    print!("(bits1 & bits2) | (bits1 ^ bits2) << 1 = %b ({})\n", (*bitwiseResult.lock().unwrap().as_mut().unwrap()), (*bitwiseResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Function calls in expressions ===".to_string());

    let mut getValue = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |n: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) * 2)));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));
    let mut getMultiplier = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(3)));
    }) as Box<dyn Fn() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));

    let mut complexResult = std::sync::Arc::new(std::sync::Mutex::new(Some((getValue.lock().unwrap().as_ref().unwrap())(a.clone()) + (getValue.lock().unwrap().as_ref().unwrap())(b.clone()) * (getMultiplier.lock().unwrap().as_ref().unwrap())() - (getValue.lock().unwrap().as_ref().unwrap())(c.clone()) / 2)));
    print!("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = {}\n", (*complexResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Array/slice expressions ===".to_string());

    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    let (mut idx1, mut idx2) = (std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
    let mut sliceResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_mut().unwrap())[(*idx1.lock().unwrap().as_mut().unwrap())..(*idx2.lock().unwrap().as_mut().unwrap())].to_vec()[1] + (*numbers.lock().unwrap().as_mut().unwrap())[(*numbers.lock().unwrap().as_mut().unwrap()).len() - 1] - (*numbers.lock().unwrap().as_mut().unwrap())[0])));
    print!("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = {}\n", (*sliceResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Map expressions ===".to_string());

    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<i32>>>>::from([("alpha".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(10)))), ("beta".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(20)))), ("gamma".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some(30))))]))));

    let mut mapResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*(*data.lock().unwrap().as_ref().unwrap()).get(&"alpha".to_string()).unwrap().lock().unwrap().as_ref().unwrap()) + (*(*data.lock().unwrap().as_ref().unwrap()).get(&"beta".to_string()).unwrap().lock().unwrap().as_ref().unwrap()) * 2 - (*(*data.lock().unwrap().as_ref().unwrap()).get(&"gamma".to_string()).unwrap().lock().unwrap().as_ref().unwrap()) / 3)));
    print!("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = {}\n", (*mapResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Struct field expressions ===".to_string());

    

    let mut p1 = Point { x: std::sync::Arc::new(std::sync::Mutex::new(Some(3))), y: std::sync::Arc::new(std::sync::Mutex::new(Some(4))) };
    let mut p2 = Point { x: std::sync::Arc::new(std::sync::Mutex::new(Some(6))), y: std::sync::Arc::new(std::sync::Mutex::new(Some(8))) };

    let mut distanceSquared = std::sync::Arc::new(std::sync::Mutex::new(Some(((*p2.lock().unwrap().as_mut().unwrap()).x - (*p1.lock().unwrap().as_mut().unwrap()).x) * ((*p2.lock().unwrap().as_mut().unwrap()).x - (*p1.lock().unwrap().as_mut().unwrap()).x) + ((*p2.lock().unwrap().as_mut().unwrap()).y - (*p1.lock().unwrap().as_mut().unwrap()).y) * ((*p2.lock().unwrap().as_mut().unwrap()).y - (*p1.lock().unwrap().as_mut().unwrap()).y))));
    print!("Distance squared between points: {}\n", (*distanceSquared.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Pointer expressions ===".to_string());

    let mut val = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut ptr = val.clone();

    let mut ptrResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*ptr.lock().unwrap().as_mut().unwrap()) + ((*ptr.lock().unwrap().as_mut().unwrap()) * 2) - ((*ptr.lock().unwrap().as_mut().unwrap()) / 2))));
    print!("*ptr + (*ptr * 2) - (*ptr / 2) = {}\n", (*ptrResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Type assertion expressions ===".to_string());

    let mut iface: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(100)));

    let (mut intVal, mut ok) = match (*iface.lock().unwrap().as_mut().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        let mut assertResult = std::sync::Arc::new(std::sync::Mutex::new(Some((*intVal.lock().unwrap().as_mut().unwrap()) * 2 + ((*intVal.lock().unwrap().as_mut().unwrap()) / 5) * 3)));
        print!("Type assertion result: {}\n", (*assertResult.lock().unwrap().as_mut().unwrap()));
    }

    println!("{}", "\n=== Channel expressions ===".to_string());

    let mut ch = ;
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt
    // TODO: Unhandled statement type: SendStmt

    let mut chanResult = std::sync::Arc::new(std::sync::Mutex::new(Some(<-(*ch.lock().unwrap().as_mut().unwrap()) + <-(*ch.lock().unwrap().as_mut().unwrap()) * 2 - <-(*ch.lock().unwrap().as_mut().unwrap()) / 2)));
    print!("Channel expression result: {}\n", (*chanResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Nested function calls ===".to_string());

    let mut add = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));
    let mut multiply = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));
    let mut subtract = std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>| -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) - (*b.lock().unwrap().as_mut().unwrap()))));
    }) as Box<dyn Fn(std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> + Send + Sync>))))));

    let mut nestedResult = (add.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some((multiply.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(3))), std::sync::Arc::new(std::sync::Mutex::new(Some(4))))))), std::sync::Arc::new(std::sync::Mutex::new(Some((subtract.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(20))), std::sync::Arc::new(std::sync::Mutex::new(Some((multiply.lock().unwrap().as_ref().unwrap())(std::sync::Arc::new(std::sync::Mutex::new(Some(2))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))))))))))));
    print!("add(multiply(3, 4), subtract(20, multiply(2, 5))) = {}\n", (*nestedResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Complex conditional expressions ===".to_string());

    let mut score = std::sync::Arc::new(std::sync::Mutex::new(Some(85)));
    let mut grade: std::sync::Arc<std::sync::Mutex<Option<String>>> = String::new();

    if (*score.lock().unwrap().as_mut().unwrap()) >= 90 {
        { let new_val = "A".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 80 {
        { let new_val = "B".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else if (*score.lock().unwrap().as_mut().unwrap()) >= 70 {
        { let new_val = "C".to_string(); *grade.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "F".to_string(); *grade.lock().unwrap() = Some(new_val); };
    }

    print!("Grade for score {}: {}\n", (*score.lock().unwrap().as_mut().unwrap()), (*grade.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Complex assignment expressions ===".to_string());

    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    { let mut guard = counter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + (5 * 3) - (10 / 2) + (8 % 3)); };
    print!("Complex assignment result: {}\n", (*counter.lock().unwrap().as_mut().unwrap()));

    let (mut sum, mut product) = (std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()) + (*c.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()) * (*c.lock().unwrap().as_mut().unwrap())))));
    print!("Sum: {}, Product: {}\n", (*sum.lock().unwrap().as_mut().unwrap()), (*product.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Range expressions ===".to_string());

    let mut total = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    for (i, val) in (*numbers.lock().unwrap().as_mut().unwrap())[..5].to_vec().iter().enumerate() {
        { let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + i * val + (val % 3)); };
    }
    print!("Complex range calculation: {}\n", (*total.lock().unwrap().as_mut().unwrap()));
}