use std::cell::{RefCell};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct CustomError {
    code: Rc<RefCell<Option<i32>>>,
    message: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.code.borrow().as_ref().unwrap()), (*self.message.borrow().as_ref().unwrap()))
    }
}


impl CustomError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Error {}: {}", (*self.code.borrow().as_ref().unwrap()), (*self.message.borrow().as_ref().unwrap())))));
    }
}

impl Error for CustomError {}


pub fn divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*b.borrow_mut().as_mut().unwrap()) == 0.0 {
        return (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return ({
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

pub fn sqrt(x: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*x.borrow_mut().as_mut().unwrap()) < 0.0 {
        return (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(Box::new(format!("cannot take square root of negative number: {}", (*x.borrow_mut().as_mut().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }

        // Simple approximation
    let mut result = Rc::new(RefCell::new(Some((*x.borrow_mut().as_mut().unwrap()) / 2.0)));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow_mut().as_mut().unwrap()) < 10 {
        { let new_val = ((*result.borrow_mut().as_mut().unwrap()) + (*x.borrow_mut().as_mut().unwrap()) / (*result.borrow_mut().as_mut().unwrap())) / 2.0; *result.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (result.clone(), Rc::new(RefCell::new(None)));
}

pub fn process_value(val: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> {

    if (*val.borrow_mut().as_mut().unwrap()) < 0 {
        return Rc::new(RefCell::new(Some(CustomError { code: Rc::new(RefCell::new(Some(100))), message: Rc::new(RefCell::new(Some("negative value not allowed".to_string()))) })));
    }
    if (*val.borrow_mut().as_mut().unwrap()) > 100 {
        return Rc::new(RefCell::new(Some(CustomError { code: Rc::new(RefCell::new(Some(200))), message: Rc::new(RefCell::new(Some("value too large".to_string()))) })));
    }
    return Rc::new(RefCell::new(None));
}

fn main() {
        // Basic error handling
    let (mut result, mut err) = divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(2.0))));
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        println!("{} {}", "10 / 2 =".to_string(), (*result.borrow_mut().as_mut().unwrap()));
    }

        // Error case
    (result, err) = divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(0.0))));
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.borrow_mut().as_mut().unwrap()));
    }

        // Formatted error
    let (mut sqrtResult, mut err) = sqrt(Rc::new(RefCell::new(Some(-4))));
    if (*err.borrow()).is_some() {
        println!("{} {}", "Sqrt error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        println!("{} {}", "Sqrt result:".to_string(), (*sqrtResult.borrow_mut().as_mut().unwrap()));
    }

        // Custom error
    { let new_val = process_value(Rc::new(RefCell::new(Some(-5)))); *err.borrow_mut() = Some(new_val); };
    if (*err.borrow()).is_some() {
        println!("{} {}", "Process error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
    }

    { let new_val = process_value(Rc::new(RefCell::new(Some(150)))); *err.borrow_mut() = Some(new_val); };
    if (*err.borrow()).is_some() {
        println!("{} {}", "Process error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
    }

    { let new_val = process_value(Rc::new(RefCell::new(Some(50)))); *err.borrow_mut() = Some(new_val); };
    if (*err.borrow()).is_some() {
        println!("{} {}", "Process error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        println!("{}", "Value processed successfully".to_string());
    }
}