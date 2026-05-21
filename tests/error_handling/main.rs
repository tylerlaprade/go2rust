use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct CustomError {
    pub code: Rc<RefCell<Option<i32>>>,
    pub message: Rc<RefCell<Option<String>>>,
}

impl CustomError {
    pub fn __go_value_clone(&self) -> Self {
        Self { code: { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, message: { let __guard = self.message.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for CustomError {
    fn default() -> Self {
        Self { code: Rc::new(RefCell::new(Some(0))), message: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl CustomError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Error {}: {}", (*self.code.borrow().as_ref().unwrap()), (*self.message.borrow().as_ref().unwrap())))));
    }
}

impl StdError for CustomError {}


pub fn divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {

    if (*b.borrow().as_ref().unwrap()) == 0.0 {
        return (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("division by zero".to_string())))));
    }
    return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

pub fn sqrt(x: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {

    if (*x.borrow().as_ref().unwrap()) < 0.0 {
        return (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("cannot take square root of negative number: {:.6}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }))))));
    }

        // Simple approximation
    let mut result = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()) / 2.0)));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 10 {
        { let new_val = ((*result.borrow().as_ref().unwrap()) + (*x.borrow().as_ref().unwrap()) / (*result.borrow().as_ref().unwrap())) / 2.0; *result.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return (Rc::new(RefCell::new(Some(result.borrow().as_ref().unwrap().clone()))), Rc::new(RefCell::new(None)));
}

pub fn process_value(val: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn StdError>>>> {

    if (*val.borrow().as_ref().unwrap()) < 0 {
        return Rc::new(RefCell::new(Some(Box::new(CustomError { code: Rc::new(RefCell::new(Some(100))), message: Rc::new(RefCell::new(Some("negative value not allowed".to_string()))), ..Default::default() }) as Box<dyn StdError>)));
    }
    if (*val.borrow().as_ref().unwrap()) > 100 {
        return Rc::new(RefCell::new(Some(Box::new(CustomError { code: Rc::new(RefCell::new(Some(200))), message: Rc::new(RefCell::new(Some("value too large".to_string()))), ..Default::default() }) as Box<dyn StdError>)));
    }
    return Rc::new(RefCell::new(None));
}

fn main() {
        // Basic error handling
    let (mut result, mut err) = divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(2.0))));
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    } else {
        println!("{} {}", format!("{}", "10 / 2 =".to_string()), format!("{}", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v }));
    }

        // Error case
    { let (__tmp_0, __tmp_1) = divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(0.0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *result.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
        let mut wrapped = Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("wrapped division: {}", format!("{}", (*err.borrow().as_ref().unwrap())))))));
        println!("{} {}", format!("{}", "Wrapped error:".to_string()), format!("{}", format!("{}", (*wrapped.borrow().as_ref().unwrap()))));
    } else {
        println!("{} {}", format!("{}", "Result:".to_string()), format!("{}", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v }));
    }

        // Formatted error
    let (mut sqrtResult, mut err) = sqrt(Rc::new(RefCell::new(Some(-4.0))));
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Sqrt error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    } else {
        println!("{} {}", format!("{}", "Sqrt result:".to_string()), format!("{}", { let __v = (*sqrtResult.borrow().as_ref().unwrap()).clone(); __v }));
    }

        // Custom error
    { let __rhs_holder = process_value(Rc::new(RefCell::new(Some(-5)))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err.borrow_mut() = new_val; };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Process error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    }

    { let __rhs_holder = process_value(Rc::new(RefCell::new(Some(150)))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err.borrow_mut() = new_val; };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Process error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    }

    { let __rhs_holder = process_value(Rc::new(RefCell::new(Some(50)))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err.borrow_mut() = new_val; };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Process error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    } else {
        println!("{}", format!("{}", "Value processed successfully".to_string()));
    }

    let mut base = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("stored error".to_string()))));
    let mut stored = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("placeholder".to_string()))));
    { let __rhs_holder = base.clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *stored.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", "Stored error:".to_string()), format!("{}", format!("{}", (*stored.borrow().as_ref().unwrap()))));
}