use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct CustomError {
    code: Arc<Mutex<Option<i32>>>,
    message: Arc<Mutex<Option<String>>>,
}

impl CustomError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("Error {}: {}", (*self.code.lock().unwrap().as_ref().unwrap()), (*self.message.lock().unwrap().as_ref().unwrap())))));
    }
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_mut().unwrap()))
    }
}

pub fn divide(a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>) -> (Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*b.lock().unwrap().as_mut().unwrap()) == 0.0 {
        return (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return ({
            let __tmp_x = (*a.lock().unwrap().as_mut().unwrap());
            let __tmp_y = (*b.lock().unwrap().as_mut().unwrap());
            Arc::new(Mutex::new(Some(__tmp_x / __tmp_y)))
        }, Arc::new(Mutex::new(None)));
}

pub fn sqrt(x: Arc<Mutex<Option<f64>>>) -> (Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*x.lock().unwrap().as_mut().unwrap()) < 0.0 {
        return (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(Box::new(format!("cannot take square root of negative number: {}", (*x.lock().unwrap().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }

        // Simple approximation
    let mut result = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) / 2.0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 10 {
        { let new_val = ((*result.lock().unwrap().as_mut().unwrap()) + (*x.lock().unwrap().as_mut().unwrap()) / (*result.lock().unwrap().as_mut().unwrap())) / 2.0; *result.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (result.clone(), Arc::new(Mutex::new(None)));
}

pub fn process_value(val: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Error + Send + Sync>>>> {

    if (*val.lock().unwrap().as_mut().unwrap()) < 0 {
        return Arc::new(Mutex::new(Some(CustomError { code: Arc::new(Mutex::new(Some(100))), message: Arc::new(Mutex::new(Some("negative value not allowed".to_string()))) })));
    }
    if (*val.lock().unwrap().as_mut().unwrap()) > 100 {
        return Arc::new(Mutex::new(Some(CustomError { code: Arc::new(Mutex::new(Some(200))), message: Arc::new(Mutex::new(Some("value too large".to_string()))) })));
    }
    return Arc::new(Mutex::new(None));
}

fn main() {
        // Basic error handling
    let (mut result, mut err) = divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(2))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "10 / 2 =".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }

        // Error case
    (result, err) = divide(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(0))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.lock().unwrap().as_mut().unwrap()));
    }

        // Formatted error
    let (mut sqrtResult, mut err) = sqrt(Arc::new(Mutex::new(Some(-4))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Sqrt error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{} {}", "Sqrt result:".to_string(), (*sqrtResult.lock().unwrap().as_mut().unwrap()));
    }

        // Custom error
    { let new_val = process_value(Arc::new(Mutex::new(Some(-5)))); *err.lock().unwrap() = Some(new_val); };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Process error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    }

    { let new_val = process_value(Arc::new(Mutex::new(Some(150)))); *err.lock().unwrap() = Some(new_val); };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Process error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    }

    { let new_val = process_value(Arc::new(Mutex::new(Some(50)))); *err.lock().unwrap() = Some(new_val); };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Process error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "Value processed successfully".to_string());
    }
}