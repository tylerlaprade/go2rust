







pub fn add(a: i32, b: i32) -> i32 {

    return a + b;
}

pub fn multiply(a: i32, b: i32) -> i32 {

    return a * b;
}

pub fn square(x: i32) -> i32 {

    return x * x;
}

pub fn is_even(x: i32) -> bool {

    return x % 2 == 0;
}

pub fn to_upper(s: String) -> String {

    let mut result = "".to_string();
    for (_, char) in s.iter().enumerate() {
        if char >= 'a' && char <= 'z' {
        result += string(char - 32);
    } else {
        result += string(char);
    }
    }
    return result;
}

pub fn apply_binary(op: BinaryOp, a: i32, b: i32) -> i32 {

    return op(a, b);
}

pub fn apply_unary(op: UnaryOp, x: i32) -> i32 {

    return op(x);
}

pub fn filter(numbers: Vec<i32>, pred: Predicate) -> Vec<i32> {

    let mut result: Vec<i32> = Default::default();
    for (_, num) in numbers.iter().enumerate() {
        if pred(num) {
        result = {result.push(num); result};
    }
    }
    return result;
}

pub fn transform(numbers: Vec<i32>, op: UnaryOp) -> Vec<i32> {

    let mut result = vec![0; numbers.len()];
    for (i, num) in numbers.iter().enumerate() {
        result[i] = op(num);
    }
    return result;
}

pub fn process_string(s: String, processor: StringProcessor) -> String {

    return processor(s);
}

pub fn make_multiplier(factor: i32) -> UnaryOp {

    return ;
}

pub fn make_adder(addend: i32) -> BinaryOp {

    return ;
}

#[derive(Debug)]
struct Calculator {
    add: BinaryOp,
    subtract: BinaryOp,
    multiply: BinaryOp,
}

fn main() {
    println!("{}", "=== Basic function types ===".to_string());
    let mut op = Default::default();
    op = add;
    print!("5 + 3 = {}\n", op(5, 3));
    op = multiply;
    print!("5 * 3 = {}\n", op(5, 3));
    println!("{}", "\n=== Higher-order functions ===".to_string());
    let mut result = apply_binary(add, 10, 20);
    print!("applyBinary(add, 10, 20) = {}\n", result);
    result = apply_binary(multiply, 4, 7);
    print!("applyBinary(multiply, 4, 7) = {}\n", result);
    let mut unaryResult = apply_unary(square, 6);
    print!("applyUnary(square, 6) = {}\n", unaryResult);
    println!("{}", "\n=== Function slices and filtering ===".to_string());
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut evens = filter(numbers, isEven);
    print!("Even numbers: {}\n", evens);
    let mut odds = filter(numbers, );
    print!("Odd numbers: {}\n", odds);
    println!("{}", "\n=== Transform operations ===".to_string());
    let mut squared = transform(vec![1, 2, 3, 4, 5], square);
    print!("Squared: {}\n", squared);
    let mut doubled = transform(vec![1, 2, 3, 4, 5], );
    print!("Doubled: {}\n", doubled);
    println!("{}", "\n=== String processing ===".to_string());
    let mut text = "hello world".to_string();
    let mut upper = process_string(text, toUpper);
    print!("'{}' -> '{}'\n", text, upper);
    let mut reversed = process_string("hello".to_string(), );
    print!("Reversed: {}\n", reversed);
    println!("{}", "\n=== Functions returning functions ===".to_string());
    let mut triple = make_multiplier(3);
    print!("triple(4) = {}\n", triple(4));
    let mut addTen = make_adder(10);
    print!("addTen(5, 3) = {}\n", add_ten(5, 3));
    println!("{}", "\n=== Struct with function fields ===".to_string());
    let mut calc = Calculator { add: , subtract: , multiply: multiply };
    print!("calc.Add(10, 5) = {}\n", calc.add(10, 5));
    print!("calc.Subtract(10, 5) = {}\n", calc.subtract(10, 5));
    print!("calc.Multiply(10, 5) = {}\n", calc.multiply(10, 5));
    println!("{}", "\n=== Function variables ===".to_string());
    let mut processor = Default::default();
    processor = toUpper;
    print!("Using toUpper: {}\n", processor("test".to_string()));
    processor = ;
    print!("Using anonymous: {}\n", processor("test".to_string()));
}