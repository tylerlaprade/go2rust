pub fn divide(a: f64, b: f64) -> (f64, Option<Box<dyn std::error::Error + Send + Sync>>) {

    if b == 0 {
        return (0, errors.new("division by zero".to_string()));
    }
    return (a / b, None);
}

pub fn sqrt(x: f64) -> (f64, Option<Box<dyn std::error::Error + Send + Sync>>) {

    if x < 0 {
        return (0, Some(Box::new(format!("cannot take square root of negative number: {}", x)) as Box<dyn std::error::Error + Send + Sync>));
    }
    let mut result = x / 2;
    let mut i = 0;
    while i < 10 {
        result =  / 2;
        i += 1;
    }
    return (result, None);
}

#[derive(Debug)]
struct CustomError {
    code: i32,
    message: String,
}

pub fn error() -> String {

    return fmt.sprintf("Error %d: %s".to_string(), e.code, e.message);
}

pub fn process_value(val: i32) -> Option<Box<dyn std::error::Error + Send + Sync>> {

    if val < 0 {
        return CustomError { code: 100, message: "negative value not allowed".to_string() };
    }
    if val > 100 {
        return CustomError { code: 200, message: "value too large".to_string() };
    }
    return None;
}

fn main() {
    let (mut result, mut err) = divide(10, 2);
    if err.is_some() {
        println!("{} {}", "Error:".to_string(), err);
    } else {
        println!("{} {}", "10 / 2 =".to_string(), result);
    }
    (result, err) = divide(10, 0);
    if err.is_some() {
        println!("{} {}", "Error:".to_string(), err);
    } else {
        println!("{} {}", "Result:".to_string(), result);
    }
    let (mut sqrtResult, mut err) = sqrt(-4);
    if err.is_some() {
        println!("{} {}", "Sqrt error:".to_string(), err);
    } else {
        println!("{} {}", "Sqrt result:".to_string(), sqrtResult);
    }
    err = process_value(-5);
    if err.is_some() {
        println!("{} {}", "Process error:".to_string(), err);
    }
    err = process_value(150);
    if err.is_some() {
        println!("{} {}", "Process error:".to_string(), err);
    }
    err = process_value(50);
    if err.is_some() {
        println!("{} {}", "Process error:".to_string(), err);
    } else {
        println!("{}", "Value processed successfully".to_string());
    }
}