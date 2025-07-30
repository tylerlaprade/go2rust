







pub fn add(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) + (*b.lock().unwrap().as_ref().unwrap()))));
}

pub fn multiply(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()) * (*b.lock().unwrap().as_ref().unwrap()))));
}

pub fn square(x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) * (*x.lock().unwrap().as_ref().unwrap()))));
}

pub fn is_even(x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<bool>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) % 2 == 0)));
}

pub fn to_upper(s: std::sync::Arc<std::sync::Mutex<Option<String>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some("".to_string())));
    for (_, char) in (*s.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if char >= 'a' && char <= 'z' {
        (*result.lock().unwrap().as_ref().unwrap()).push_str(&string(std::sync::Arc::new(std::sync::Mutex::new(Some(char - 32)))));
    } else {
        (*result.lock().unwrap().as_ref().unwrap()).push_str(&string(std::sync::Arc::new(std::sync::Mutex::new(Some(char)))));
    }
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_ref().unwrap()))));
}

pub fn apply_binary(op: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>, a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(op(std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()))))))));
}

pub fn apply_unary(op: std::sync::Arc<std::sync::Mutex<Option<UnaryOp>>>, x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(op(std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()))))))));
}

pub fn filter(numbers: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, pred: std::sync::Arc<std::sync::Mutex<Option<Predicate>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> {

    let mut result: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(Default::default())));
    for (_, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        if pred(std::sync::Arc::new(std::sync::Mutex::new(Some(num)))) {
        { let new_val = {(*result.lock().unwrap().as_ref().unwrap()).push(num); (*result.lock().unwrap().as_ref().unwrap())}; *result.lock().unwrap() = Some(new_val); };
    }
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_ref().unwrap()))));
}

pub fn transform(numbers: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, op: std::sync::Arc<std::sync::Mutex<Option<UnaryOp>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; (*numbers.lock().unwrap().as_ref().unwrap()).len()])));
    for (i, num) in (*numbers.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[i] = op(std::sync::Arc::new(std::sync::Mutex::new(Some(num))));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_ref().unwrap()))));
}

pub fn process_string(s: std::sync::Arc<std::sync::Mutex<Option<String>>>, processor: std::sync::Arc<std::sync::Mutex<Option<StringProcessor>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(processor(std::sync::Arc::new(std::sync::Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()))))))));
}

pub fn make_multiplier(factor: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<UnaryOp>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some()));
}

pub fn make_adder(addend: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some()));
}

#[derive(Debug)]
struct Calculator {
    add: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>,
    subtract: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>,
    multiply: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>,
}

fn main() {
    println!("{}", "=== Basic function types ===".to_string());
    let mut op = Default::default();
    { let new_val = (*add.lock().unwrap().as_ref().unwrap()); *op.lock().unwrap() = Some(new_val); };
    print!("5 + 3 = {}\n", op(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))));
    { let new_val = (*multiply.lock().unwrap().as_ref().unwrap()); *op.lock().unwrap() = Some(new_val); };
    print!("5 * 3 = {}\n", op(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))));
    println!("{}", "\n=== Higher-order functions ===".to_string());
    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some(apply_binary(std::sync::Arc::new(std::sync::Mutex::new(Some((*add.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20)))))));
    print!("applyBinary(add, 10, 20) = {}\n", (*result.lock().unwrap().as_ref().unwrap()));
    { let new_val = apply_binary(std::sync::Arc::new(std::sync::Mutex::new(Some((*multiply.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(7)))); *result.lock().unwrap() = Some(new_val); };
    print!("applyBinary(multiply, 4, 7) = {}\n", (*result.lock().unwrap().as_ref().unwrap()));
    let mut unaryResult = std::sync::Arc::new(std::sync::Mutex::new(Some(apply_unary(std::sync::Arc::new(std::sync::Mutex::new(Some((*square.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some(6)))))));
    print!("applyUnary(square, 6) = {}\n", (*unaryResult.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Function slices and filtering ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
    let mut evens = std::sync::Arc::new(std::sync::Mutex::new(Some(filter(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*isEven.lock().unwrap().as_ref().unwrap()))))))));
    print!("Even numbers: {}\n", (*evens.lock().unwrap().as_ref().unwrap()));
    let mut odds = std::sync::Arc::new(std::sync::Mutex::new(Some(filter(std::sync::Arc::new(std::sync::Mutex::new(Some((*numbers.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some()))))));
    print!("Odd numbers: {}\n", (*odds.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Transform operations ===".to_string());
    let mut squared = std::sync::Arc::new(std::sync::Mutex::new(Some(transform(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5]))), std::sync::Arc::new(std::sync::Mutex::new(Some((*square.lock().unwrap().as_ref().unwrap()))))))));
    print!("Squared: {}\n", (*squared.lock().unwrap().as_ref().unwrap()));
    let mut doubled = std::sync::Arc::new(std::sync::Mutex::new(Some(transform(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5]))), std::sync::Arc::new(std::sync::Mutex::new(Some()))))));
    print!("Doubled: {}\n", (*doubled.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== String processing ===".to_string());
    let mut text = std::sync::Arc::new(std::sync::Mutex::new(Some("hello world".to_string())));
    let mut upper = std::sync::Arc::new(std::sync::Mutex::new(Some(process_string(std::sync::Arc::new(std::sync::Mutex::new(Some((*text.lock().unwrap().as_ref().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*toUpper.lock().unwrap().as_ref().unwrap()))))))));
    print!("'{}' -> '{}'\n", (*text.lock().unwrap().as_ref().unwrap()), (*upper.lock().unwrap().as_ref().unwrap()));
    let mut reversed = std::sync::Arc::new(std::sync::Mutex::new(Some(process_string(std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some()))))));
    print!("Reversed: {}\n", (*reversed.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Functions returning functions ===".to_string());
    let mut triple = std::sync::Arc::new(std::sync::Mutex::new(Some(make_multiplier(std::sync::Arc::new(std::sync::Mutex::new(Some(3)))))));
    print!("triple(4) = {}\n", triple(std::sync::Arc::new(std::sync::Mutex::new(Some(4)))));
    let mut addTen = std::sync::Arc::new(std::sync::Mutex::new(Some(make_adder(std::sync::Arc::new(std::sync::Mutex::new(Some(10)))))));
    print!("addTen(5, 3) = {}\n", add_ten(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))));
    println!("{}", "\n=== Struct with function fields ===".to_string());
    let mut calc = std::sync::Arc::new(std::sync::Mutex::new(Some(Calculator { add: , subtract: , multiply: (*multiply.lock().unwrap().as_ref().unwrap()) })));
    print!("calc.Add(10, 5) = {}\n", (*calc.lock().unwrap().as_ref().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))));
    print!("calc.Subtract(10, 5) = {}\n", (*calc.lock().unwrap().as_ref().unwrap()).subtract(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))));
    print!("calc.Multiply(10, 5) = {}\n", (*calc.lock().unwrap().as_ref().unwrap()).multiply(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))));
    println!("{}", "\n=== Function variables ===".to_string());
    let mut processor = Default::default();
    { let new_val = (*toUpper.lock().unwrap().as_ref().unwrap()); *processor.lock().unwrap() = Some(new_val); };
    print!("Using toUpper: {}\n", processor(std::sync::Arc::new(std::sync::Mutex::new(Some("test".to_string())))));
    { let new_val = ; *processor.lock().unwrap() = Some(new_val); };
    print!("Using anonymous: {}\n", processor(std::sync::Arc::new(std::sync::Mutex::new(Some("test".to_string())))));
}