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

/// Function type definitions
// TODO: Unhandled type declaration: FuncType
type BinaryOp = std::sync::Arc<std::sync::Mutex<Option<()>>>

// TODO: Unhandled type declaration: FuncType
type UnaryOp = std::sync::Arc<std::sync::Mutex<Option<()>>>

// TODO: Unhandled type declaration: FuncType
type Predicate = std::sync::Arc<std::sync::Mutex<Option<()>>>

// TODO: Unhandled type declaration: FuncType
type StringProcessor = std::sync::Arc<std::sync::Mutex<Option<()>>>

/// Struct with function fields
#[derive(Debug)]
struct Calculator {
    add: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>,
    subtract: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>,
    multiply: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>,
}

/// Functions that match the types
pub fn add(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
}

pub fn multiply(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
}

pub fn square(x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * (*x.lock().unwrap().as_mut().unwrap()))));
}

pub fn is_even(x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<bool>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) % 2 == 0)));
}

pub fn to_upper(s: std::sync::Arc<std::sync::Mutex<Option<String>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some("".to_string())));
    for (_, char) in (*(*s.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).chars().enumerate() {
        if char >= 'a' && char <= 'z' {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&string(std::sync::Arc::new(std::sync::Mutex::new(Some(char - 32)))));
    } else {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&string(std::sync::Arc::new(std::sync::Mutex::new(Some(char)))));
    }
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap()).clone())));
}

/// Higher-order functions
pub fn apply_binary(op: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>>, a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(op(a.clone(), b.clone()))));
}

pub fn apply_unary(op: std::sync::Arc<std::sync::Mutex<Option<UnaryOp>>>, x: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(op(x.clone()))));
}

pub fn filter(numbers: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, pred: std::sync::Arc<std::sync::Mutex<Option<Predicate>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> {

    let mut result: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(Default::default())));
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        if pred(std::sync::Arc::new(std::sync::Mutex::new(Some(num)))) {
        {(*result.lock().unwrap().as_mut().unwrap()).push(num); result.clone()};
    }
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap()).clone())));
}

pub fn transform(numbers: std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>>, op: std::sync::Arc<std::sync::Mutex<Option<UnaryOp>>>) -> std::sync::Arc<std::sync::Mutex<Option<Vec<i32>>>> {

    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![0; (*numbers.lock().unwrap().as_mut().unwrap()).len()])));
    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[i] = op(std::sync::Arc::new(std::sync::Mutex::new(Some(num))));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap()).clone())));
}

pub fn process_string(s: std::sync::Arc<std::sync::Mutex<Option<String>>>, processor: std::sync::Arc<std::sync::Mutex<Option<StringProcessor>>>) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(processor(s.clone()))));
}

/// Function that returns a function
pub fn make_multiplier(factor: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<UnaryOp>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(()))))));
}

pub fn make_adder(addend: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(()))))));
}

fn main() {
    println!("{}", "=== Basic function types ===".to_string());

    let mut op: std::sync::Arc<std::sync::Mutex<Option<BinaryOp>>> = Default::default();
    { let new_val = (*add.lock().unwrap().as_mut().unwrap()); *op.lock().unwrap() = Some(new_val); };
    print!("5 + 3 = {}\n", (*op(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_mut().unwrap()));

    { let new_val = (*multiply.lock().unwrap().as_mut().unwrap()); *op.lock().unwrap() = Some(new_val); };
    print!("5 * 3 = {}\n", (*op(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Higher-order functions ===".to_string());
    let mut result = apply_binary(add.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(20))));
    print!("applyBinary(add, 10, 20) = {}\n", (*result.lock().unwrap().as_mut().unwrap()));

    { let new_val = apply_binary(multiply.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(4))), std::sync::Arc::new(std::sync::Mutex::new(Some(7)))); *result.lock().unwrap() = Some(new_val); };
    print!("applyBinary(multiply, 4, 7) = {}\n", (*result.lock().unwrap().as_mut().unwrap()));

    let mut unaryResult = apply_unary(square.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(6))));
    print!("applyUnary(square, 6) = {}\n", (*unaryResult.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Function slices and filtering ===".to_string());
    let mut numbers = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    let mut evens = filter(numbers.clone(), isEven.clone());
    print!("Even numbers: {}\n", format_slice(&evens));

    let mut odds = filter(numbers.clone(), std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(())))))));
    print!("Odd numbers: {}\n", format_slice(&odds));

    println!("{}", "\n=== Transform operations ===".to_string());
    let mut squared = transform(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])))))), square.clone());
    print!("Squared: {}\n", format_slice(&squared));

    let mut doubled = transform(std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![1, 2, 3, 4, 5])))))), std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(())))))));
    print!("Doubled: {}\n", format_slice(&doubled));

    println!("{}", "\n=== String processing ===".to_string());
    let mut text = std::sync::Arc::new(std::sync::Mutex::new(Some("hello world".to_string())));
    let mut upper = process_string(text.clone(), toUpper.clone());
    print!("'{}' -> '{}'\n", (*text.lock().unwrap().as_mut().unwrap()), (*upper.lock().unwrap().as_mut().unwrap()));

    let mut reversed = process_string(std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(())))))));
    print!("Reversed: {}\n", (*reversed.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Functions returning functions ===".to_string());
    let mut triple = make_multiplier(std::sync::Arc::new(std::sync::Mutex::new(Some(3))));
    print!("triple(4) = {}\n", (*triple(std::sync::Arc::new(std::sync::Mutex::new(Some(4)))).lock().unwrap().as_mut().unwrap()));

    let mut addTen = make_adder(std::sync::Arc::new(std::sync::Mutex::new(Some(10))));
    print!("addTen(5, 3) = {}\n", (*add_ten(std::sync::Arc::new(std::sync::Mutex::new(Some(5))), std::sync::Arc::new(std::sync::Mutex::new(Some(3)))).lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Struct with function fields ===".to_string());
    let mut calc = Calculator { add: std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(())))))), subtract: std::sync::Arc::new(std::sync::Mutex::new(Some(/* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(())))))), multiply: std::sync::Arc::new(std::sync::Mutex::new(Some((*multiply.lock().unwrap().as_mut().unwrap())))) };

    print!("calc.Add(10, 5) = {}\n", (*(*calc.lock().unwrap().as_mut().unwrap()).add(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    print!("calc.Subtract(10, 5) = {}\n", (*(*calc.lock().unwrap().as_mut().unwrap()).subtract(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));
    print!("calc.Multiply(10, 5) = {}\n", (*(*calc.lock().unwrap().as_mut().unwrap()).multiply(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(5)))).lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Function variables ===".to_string());
    let mut processor: std::sync::Arc<std::sync::Mutex<Option<StringProcessor>>> = Default::default();
    { let new_val = (*toUpper.lock().unwrap().as_mut().unwrap()); *processor.lock().unwrap() = Some(new_val); };
    print!("Using toUpper: {}\n", (*processor(std::sync::Arc::new(std::sync::Mutex::new(Some("test".to_string())))).lock().unwrap().as_mut().unwrap()));

    { let new_val = /* TODO: Unhandled expression type: FuncLit */ std::sync::Arc::new(std::sync::Mutex::new(Some(()))); *processor.lock().unwrap() = Some(new_val); };
    print!("Using anonymous: {}\n", (*processor(std::sync::Arc::new(std::sync::Mutex::new(Some("test".to_string())))).lock().unwrap().as_mut().unwrap()));
}