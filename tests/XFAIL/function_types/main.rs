use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
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
#[derive(Debug, Clone)]
struct BinaryOp(Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>>);


#[derive(Debug, Clone)]
struct UnaryOp(Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>>);


#[derive(Debug, Clone)]
struct Predicate(Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<bool>>> + Send + Sync>>>>);


#[derive(Debug, Clone)]
struct StringProcessor(Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>);


/// Struct with function fields
#[derive(Debug)]
struct Calculator {
    add: Arc<Mutex<Option<BinaryOp>>>,
    subtract: Arc<Mutex<Option<BinaryOp>>>,
    multiply: Arc<Mutex<Option<BinaryOp>>>,
}

/// Functions that match the types
pub fn add(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
}

pub fn multiply(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn square(x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*x.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
}

pub fn is_even(x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<bool>>> {

    return Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) % 2 == 0)));
}

pub fn to_upper(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {

    let mut result = Arc::new(Mutex::new(Some("".to_string())));
    for (_, char) in (*(*s.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).chars().enumerate() {
        if char >= ('a' as i32) && char <= ('z' as i32) {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&Arc::new(Mutex::new(Some(char::from_u32((*char - 32.lock().unwrap().as_ref().unwrap()) as u32).unwrap().to_string()))));
    } else {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&Arc::new(Mutex::new(Some(char::from_u32((*char.lock().unwrap().as_ref().unwrap()) as u32).unwrap().to_string()))));
    }
    }
    return result.clone();
}

/// Higher-order functions
pub fn apply_binary(op: Arc<Mutex<Option<BinaryOp>>>, a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some((op.lock().unwrap().as_ref().unwrap())(a.clone(), b.clone()))));
}

pub fn apply_unary(op: Arc<Mutex<Option<UnaryOp>>>, x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some((op.lock().unwrap().as_ref().unwrap())(x.clone()))));
}

pub fn filter(numbers: Arc<Mutex<Option<Vec<i32>>>>, pred: Arc<Mutex<Option<Predicate>>>) -> Arc<Mutex<Option<Vec<i32>>>> {

    let mut result: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(Some(Default::default())));
    for num in &(*numbers.lock().unwrap().as_mut().unwrap()) {
        if (pred.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(num)))) {
        {(*result.lock().unwrap().as_mut().unwrap()).push(num); result.clone()};
    }
    }
    return result.clone();
}

pub fn transform(numbers: Arc<Mutex<Option<Vec<i32>>>>, op: Arc<Mutex<Option<UnaryOp>>>) -> Arc<Mutex<Option<Vec<i32>>>> {

    let mut result = Arc::new(Mutex::new(Some(vec![0; (*numbers.lock().unwrap().as_ref().unwrap()).len()])));
    for (i, num) in (*numbers.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        (*result.lock().unwrap().as_mut().unwrap())[i] = (op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(num))));
    }
    return result.clone();
}

pub fn process_string(s: Arc<Mutex<Option<String>>>, processor: Arc<Mutex<Option<StringProcessor>>>) -> Arc<Mutex<Option<String>>> {

    return Arc::new(Mutex::new(Some((processor.lock().unwrap().as_ref().unwrap())(s.clone()))));
}

/// Function that returns a function
pub fn make_multiplier(factor: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<UnaryOp>>> {

    return Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*factor.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

pub fn make_adder(addend: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<BinaryOp>>> {

    return Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*addend.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)));
}

fn main() {
        // Basic function type usage
    println!("{}", "=== Basic function types ===".to_string());

    let mut op: Arc<Mutex<Option<BinaryOp>>> = Default::default();
    { let new_val = (*add.lock().unwrap().as_mut().unwrap()); *op.lock().unwrap() = Some(new_val); };
    print!("5 + 3 = {}\n", (*(op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));

    { let new_val = (*multiply.lock().unwrap().as_mut().unwrap()); *op.lock().unwrap() = Some(new_val); };
    print!("5 * 3 = {}\n", (*(op.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));

        // Higher-order functions
    println!("{}", "\n=== Higher-order functions ===".to_string());
    let mut result = apply_binary(add.clone(), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(20))));
    print!("applyBinary(add, 10, 20) = {}\n", (*result.lock().unwrap().as_mut().unwrap()));

    { let new_val = apply_binary(multiply.clone(), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(7)))); *result.lock().unwrap() = Some(new_val); };
    print!("applyBinary(multiply, 4, 7) = {}\n", (*result.lock().unwrap().as_mut().unwrap()));

    let mut unaryResult = apply_unary(square.clone(), Arc::new(Mutex::new(Some(6))));
    print!("applyUnary(square, 6) = {}\n", (*unaryResult.lock().unwrap().as_mut().unwrap()));

        // Function slices and filtering
    println!("{}", "\n=== Function slices and filtering ===".to_string());
    let mut numbers = Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));

    let mut evens = filter(numbers.clone(), isEven.clone());
    print!("Even numbers: {}\n", format_slice(&evens));

    let mut odds = filter(numbers.clone(), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<bool>>> {
        return Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) % 2 != 0)));
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<bool>>> + Send + Sync>)))))));
    print!("Odd numbers: {}\n", format_slice(&odds));

        // Transform with function types
    println!("{}", "\n=== Transform operations ===".to_string());
    let mut squared = transform(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])))))), square.clone());
    print!("Squared: {}\n", format_slice(&squared));

    let mut doubled = transform(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec![1, 2, 3, 4, 5])))))), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*x.lock().unwrap().as_mut().unwrap());
            let __tmp_y = 2;
            Arc::new(Mutex::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))))));
    print!("Doubled: {}\n", format_slice(&doubled));

        // String processing
    println!("{}", "\n=== String processing ===".to_string());
    let mut text = Arc::new(Mutex::new(Some("hello world".to_string())));
    let mut upper = process_string(text.clone(), toUpper.clone());
    print!("'{}' -> '{}'\n", (*text.lock().unwrap().as_mut().unwrap()), (*upper.lock().unwrap().as_mut().unwrap()));

    let mut reversed = process_string(Arc::new(Mutex::new(Some("hello".to_string()))), Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> {
        let mut runes = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>())));
        let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some((*runes.lock().unwrap().as_ref().unwrap()).len() - 1))));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*j.lock().unwrap().as_mut().unwrap()) {
        { *(*runes.lock().unwrap().as_ref().unwrap())[(*i.lock().unwrap().as_mut().unwrap()) as usize].clone().lock().unwrap() = Some((*runes.lock().unwrap().as_ref().unwrap())[(*j.lock().unwrap().as_mut().unwrap()) as usize].clone()); *(*runes.lock().unwrap().as_ref().unwrap())[(*j.lock().unwrap().as_mut().unwrap()) as usize].clone().lock().unwrap() = Some((*runes.lock().unwrap().as_ref().unwrap())[(*i.lock().unwrap().as_mut().unwrap()) as usize].clone()) };
        { *(*i.lock().unwrap().as_mut().unwrap()).lock().unwrap() = Some((*i.lock().unwrap().as_mut().unwrap()) + 1); *(*j.lock().unwrap().as_mut().unwrap()).lock().unwrap() = Some((*j.lock().unwrap().as_mut().unwrap()) - 1) };
    }
        return Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*runes.lock().unwrap().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()))))));
    }) as Box<dyn Fn(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)))))));
    print!("Reversed: {}\n", (*reversed.lock().unwrap().as_mut().unwrap()));

        // Functions that return functions
    println!("{}", "\n=== Functions returning functions ===".to_string());
    let mut triple = make_multiplier(Arc::new(Mutex::new(Some(3))));
    print!("triple(4) = {}\n", (*(triple.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(4)))).lock().unwrap().as_ref().unwrap()));

    let mut addTen = make_adder(Arc::new(Mutex::new(Some(10))));
    print!("addTen(5, 3) = {}\n", (*(addTen.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()));

        // Struct with function fields
    println!("{}", "\n=== Struct with function fields ===".to_string());
    let mut calc = Calculator { add: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))))), subtract: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<i32>>> {
        return {
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>)))))), multiply: multiply.clone() };

        // Reuse existing function
    print!("calc.Add(10, 5) = {}\n", (*calc.add(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));
    print!("calc.Subtract(10, 5) = {}\n", (*calc.subtract(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));
    print!("calc.Multiply(10, 5) = {}\n", (*calc.multiply(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(5)))).lock().unwrap().as_ref().unwrap()));

        // Function variables
    println!("{}", "\n=== Function variables ===".to_string());
    let mut processor: Arc<Mutex<Option<StringProcessor>>> = Default::default();
    { let new_val = (*toUpper.lock().unwrap().as_mut().unwrap()); *processor.lock().unwrap() = Some(new_val); };
    print!("Using toUpper: {}\n", (*(processor.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some("test".to_string())))).lock().unwrap().as_ref().unwrap()));

    { let new_val = Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<String>>> {
        return {
            let __tmp_x = "processed: ".to_string();
            let __tmp_y = (*s.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>))); *processor.lock().unwrap() = Some(new_val); };
    print!("Using anonymous: {}\n", (*(processor.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some("test".to_string())))).lock().unwrap().as_ref().unwrap()));
}