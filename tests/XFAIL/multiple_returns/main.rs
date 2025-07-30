pub fn divmod(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) / (*b.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) % (*b.lock().unwrap().as_ref().unwrap())))));
}

pub fn parse_number(s: std::sync::Arc<std::sync::Mutex<Option<String>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    let (mut num, mut err) = match (*s.lock().unwrap().as_ref().unwrap()).parse::<i32>() { Ok(n) => (std::sync::Arc::new(std::sync::Mutex::new(Some(n))), std::sync::Arc::new(std::sync::Mutex::new(None))), Err(e) => (std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))) };
    if (*err.lock().unwrap().as_ref().unwrap()).is_some() {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("failed to parse '{}': {}", (*s.lock().unwrap().as_ref().unwrap()), (*err.lock().unwrap().as_ref().unwrap()))) as Box<dyn std::error::Error + Send + Sync>)))))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*num.lock().unwrap().as_ref().unwrap()).clone()))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn get_name_age() -> (std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(30))));
}

pub fn calculate(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    let mut sum: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut product: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));

    { let new_val = (*a.lock().unwrap().as_ref().unwrap()) + (*b.lock().unwrap().as_ref().unwrap()); *sum.lock().unwrap() = Some(new_val); };
    { let new_val = (*a.lock().unwrap().as_ref().unwrap()) * (*b.lock().unwrap().as_ref().unwrap()); *product.lock().unwrap() = Some(new_val); };
    return (sum, product);
}

pub fn process_data(data: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {
    let mut min: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut max: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut sum: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));

    if (*data.lock().unwrap().as_ref().unwrap()).len() == 0 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(0))));
    }
    { let new_val = (*data.lock().unwrap().as_ref().unwrap())[0]; *min.lock().unwrap() = Some(new_val); };
    { let new_val = (*data.lock().unwrap().as_ref().unwrap())[0]; *max.lock().unwrap() = Some(new_val); };
    { let new_val = 0; *sum.lock().unwrap() = Some(new_val); };
    for (_, val) in (*data.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if val < (*min.lock().unwrap().as_ref().unwrap()) {
        { let new_val = val; *min.lock().unwrap() = Some(new_val); };
    }
        if val > (*max.lock().unwrap().as_ref().unwrap()) {
        { let new_val = val; *max.lock().unwrap() = Some(new_val); };
    }
        (*sum.lock().unwrap().as_ref().unwrap()) += val;
    }
    return (min, max, sum);
}

pub fn swap(a: std::sync::Arc<std::sync::Mutex<Option<String>>>, b: std::sync::Arc<std::sync::Mutex<Option<String>>>) -> (std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<String>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()).clone()))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()).clone()))));
}

pub fn get_person_info() -> (std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<f64>>>, std::sync::Arc<std::sync::Mutex<Option<bool>>>) {
    let mut name: std::sync::Arc<std::sync::Mutex<Option<String>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(String::new())));
    let mut age: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut height: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0.0)));
    let mut married: std::sync::Arc<std::sync::Mutex<Option<bool>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(false)));

    return (std::sync::Arc::new(std::sync::Mutex::new(Some("Bob".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(25))), std::sync::Arc::new(std::sync::Mutex::new(Some(5.9))), std::sync::Arc::new(std::sync::Mutex::new(Some(false))));
}

pub fn find_in_slice(slice: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, target: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<bool>>>) {
    let mut index: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut found: std::sync::Arc<std::sync::Mutex<Option<bool>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(false)));

    for (i, val) in (*slice.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if val == (*target.lock().unwrap().as_ref().unwrap()) {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(i))), std::sync::Arc::new(std::sync::Mutex::new(Some(true))));
    }
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some(-1))), std::sync::Arc::new(std::sync::Mutex::new(Some(false))));
}

pub fn safe_divide(a: std::sync::Arc<std::sync::Mutex<Option<f64>>>, b: std::sync::Arc<std::sync::Mutex<Option<f64>>>) -> (std::sync::Arc<std::sync::Mutex<Option<f64>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {
    let mut result: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0.0)));
    let mut err: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> = std::sync::Arc::new(std::sync::Mutex::new(None));

    if (*b.lock().unwrap().as_ref().unwrap()) == 0 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("division by zero")) as Box<dyn std::error::Error + Send + Sync>)))))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) / (*b.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

fn main() {
    println!("{}", "=== Basic multiple returns ===".to_string());
    let (mut quotient, mut remainder) = divmod(std::sync::Arc::new(std::sync::Mutex::new(Some(17))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    print!("17 / 5 = {} remainder {}\n", (*quotient.lock().unwrap().as_ref().unwrap()), (*remainder.lock().unwrap().as_ref().unwrap()));
    let (mut name, mut age) = get_name_age();
    print!("Name: {}, Age: {}\n", (*name.lock().unwrap().as_ref().unwrap()), (*age.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Multiple returns with errors ===".to_string());
    let (mut num, mut err) = parse_number(std::sync::Arc::new(std::sync::Mutex::new(Some("123".to_string()))));
    if (*err.lock().unwrap().as_ref().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_ref().unwrap()));
    } else {
        print!("Parsed number: {}\n", (*num.lock().unwrap().as_ref().unwrap()));
    }
    (num, err) = parse_number(std::sync::Arc::new(std::sync::Mutex::new(Some("abc".to_string()))));
    if (*err.lock().unwrap().as_ref().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_ref().unwrap()));
    } else {
        print!("Parsed number: {}\n", (*num.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "\n=== Named return values ===".to_string());
    let (mut s, mut p) = calculate(std::sync::Arc::new(std::sync::Mutex::new(Some(6))), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
    print!("Sum: {}, Product: {}\n", (*s.lock().unwrap().as_ref().unwrap()), (*p.lock().unwrap().as_ref().unwrap()));
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![3, 1, 4, 1, 5, 9, 2, 6])));
    let (mut min, mut max, mut sum) = process_data(std::sync::Arc::new(std::sync::Mutex::new(Some((*data.lock().unwrap().as_ref().unwrap())))));
    print!("Data: {}\n", (*data.lock().unwrap().as_ref().unwrap()));
    print!("Min: {}, Max: {}, Sum: {}\n", (*min.lock().unwrap().as_ref().unwrap()), (*max.lock().unwrap().as_ref().unwrap()), (*sum.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Swapping values ===".to_string());
    let (mut (*x.lock().unwrap().as_ref().unwrap()), mut (*y.lock().unwrap().as_ref().unwrap())) = ("hello".to_string(), "world".to_string());
    print!("Before swap: x={}, y={}\n", (*x.lock().unwrap().as_ref().unwrap()), (*y.lock().unwrap().as_ref().unwrap()));
    (x, y) = swap(std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap())))));
    print!("After swap: x={}, y={}\n", (*x.lock().unwrap().as_ref().unwrap()), (*y.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Different types ===".to_string());
    let (mut pName, mut pAge, mut pHeight, mut pMarried) = get_person_info();
    print!("Person: {}, {} years old, {:.1} feet tall, married: {}\n", (*pName.lock().unwrap().as_ref().unwrap()), (*pAge.lock().unwrap().as_ref().unwrap()), (*pHeight.lock().unwrap().as_ref().unwrap()), (*pMarried.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Finding in slice ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![10, 20, 30, 40, 50])));
    let (mut index, mut found) = find_in_slice(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(30))));
    if (*found.lock().unwrap().as_ref().unwrap()) {
        print!("Found 30 at index {}\n", (*index.lock().unwrap().as_ref().unwrap()));
    } else {
        println!("{}", "30 not found".to_string());
    }
    (index, found) = find_in_slice(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(99))));
    if (*found.lock().unwrap().as_ref().unwrap()) {
        print!("Found 99 at index {}\n", (*index.lock().unwrap().as_ref().unwrap()));
    } else {
        println!("{}", "99 not found".to_string());
    }
    println!("{}", "\n=== Safe division ===".to_string());
    let (mut result, mut err) = safe_divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10.0))), std::sync::Arc::new(std::sync::Mutex::new(Some(3.0))));
    if (*err.lock().unwrap().as_ref().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_ref().unwrap()));
    } else {
        print!("10.0 / 3.0 = {:.2}\n", (*result.lock().unwrap().as_ref().unwrap()));
    }
    (result, err) = safe_divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10.0))), std::sync::Arc::new(std::sync::Mutex::new(Some(0.0))));
    if (*err.lock().unwrap().as_ref().unwrap()).is_some() {
        print!("Error: {}\n", (*err.lock().unwrap().as_ref().unwrap()));
    } else {
        print!("Result: {:.2}\n", (*result.lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "\n=== Ignoring return values ===".to_string());
    let (_, mut remainder2) = divmod(std::sync::Arc::new(std::sync::Mutex::new(Some(23))), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
    print!("23 mod 7 = {} (quotient ignored)\n", (*remainder2.lock().unwrap().as_ref().unwrap()));
    let (mut name2, _) = get_name_age();
    print!("Name only: {} (age ignored)\n", (*name2.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Multiple assignment ===".to_string());
    let (mut (*a.lock().unwrap().as_ref().unwrap()), mut (*b.lock().unwrap().as_ref().unwrap()), mut (*c.lock().unwrap().as_ref().unwrap())) = (1, 2, 3);
    print!("a={}, b={}, c={}\n", (*a.lock().unwrap().as_ref().unwrap()), (*b.lock().unwrap().as_ref().unwrap()), (*c.lock().unwrap().as_ref().unwrap()));
    { *(*a.lock().unwrap().as_ref().unwrap()).lock().unwrap() = Some((*b.lock().unwrap().as_ref().unwrap())); *(*b.lock().unwrap().as_ref().unwrap()).lock().unwrap() = Some((*a.lock().unwrap().as_ref().unwrap())) };
    print!("After swap: a={}, b={}\n", (*a.lock().unwrap().as_ref().unwrap()), (*b.lock().unwrap().as_ref().unwrap()));
}