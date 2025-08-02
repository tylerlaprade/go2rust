#[derive(Debug)]
struct CustomError {
    code: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    message: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl CustomError {
    pub fn error(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("Error {}: {}", (*self.code.lock().unwrap().as_mut().unwrap()), (*self.message.lock().unwrap().as_mut().unwrap())))));
    }
}

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_mut().unwrap()))
    }
}

pub fn divide(a: std::sync::Arc<std::sync::Mutex<Option<f64>>>, b: std::sync::Arc<std::sync::Mutex<Option<f64>>>) -> (std::sync::Arc<std::sync::Mutex<Option<f64>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn sqrt(x: std::sync::Arc<std::sync::Mutex<Option<f64>>>) -> (std::sync::Arc<std::sync::Mutex<Option<f64>>>, std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>) {

    if (*x.lock().unwrap().as_mut().unwrap()) < 0 {
        return (std::sync::Arc::new(std::sync::Mutex::new(Some(0))), std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(format!("cannot take square root of negative number: {}", (*x.lock().unwrap().as_mut().unwrap()))) as Box<dyn std::error::Error + Send + Sync>))));
    }
    let mut result = std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) / 2)));
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 10 {
        { let new_val = ((*result.lock().unwrap().as_mut().unwrap()) + (*x.lock().unwrap().as_mut().unwrap()) / (*result.lock().unwrap().as_mut().unwrap())) / 2; *result.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*result.lock().unwrap().as_mut().unwrap()).clone()))), std::sync::Arc::new(std::sync::Mutex::new(None)));
}

pub fn process_value(val: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>> {

    if (*val.lock().unwrap().as_mut().unwrap()) < 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(CustomError { code: std::sync::Arc::new(std::sync::Mutex::new(Some(100))), message: std::sync::Arc::new(std::sync::Mutex::new(Some("negative value not allowed".to_string()))) })));
    }
    if (*val.lock().unwrap().as_mut().unwrap()) > 100 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(CustomError { code: std::sync::Arc::new(std::sync::Mutex::new(Some(200))), message: std::sync::Arc::new(std::sync::Mutex::new(Some("value too large".to_string()))) })));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(None));
}

fn main() {
    let (mut result, mut err) = divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "10 / 2 =".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }
    (result, err) = divide(std::sync::Arc::new(std::sync::Mutex::new(Some(10))), std::sync::Arc::new(std::sync::Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }
    let (mut sqrtResult, mut err) = sqrt(std::sync::Arc::new(std::sync::Mutex::new(Some(-4))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Sqrt error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Sqrt result:".to_string(), (*sqrtResult.lock().unwrap().as_mut().unwrap()));
    }
    { let new_val = process_value(std::sync::Arc::new(std::sync::Mutex::new(Some(-5)))); *err.lock().unwrap() = Some(new_val); };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Process error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    }
    { let new_val = process_value(std::sync::Arc::new(std::sync::Mutex::new(Some(150)))); *err.lock().unwrap() = Some(new_val); };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Process error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    }
    { let new_val = process_value(std::sync::Arc::new(std::sync::Mutex::new(Some(50)))); *err.lock().unwrap() = Some(new_val); };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Process error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "Value processed successfully".to_string());
    }
}