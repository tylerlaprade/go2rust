use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<Rc<dyn StdError>>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<Rc<dyn StdError>>>) -> Rc<dyn StdError> {
    let many = wrapped.len() > 1;
    Rc::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as Rc<dyn StdError>
}

fn go_errors_join(errs: Vec<Option<Rc<dyn StdError>>>) -> Option<Rc<dyn StdError>> {
    let errs: Vec<Rc<dyn StdError>> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(Rc::new(GoWrapError { msg, wrapped: errs, many: true }) as Rc<dyn StdError>)
}

// err and every error it wraps, depth first, in the order errors.Is and
// errors.As visit them.
fn go_error_chain(err: Option<Rc<dyn StdError>>) -> Vec<Rc<dyn StdError>> {
    let mut out = Vec::new();
    let mut stack: Vec<Rc<dyn StdError>> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.iter().rev().cloned());
        }
        out.push(e);
    }
    out
}

fn go_errors_unwrap(err: Option<Rc<dyn StdError>>) -> Option<Rc<dyn StdError>> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.first().cloned() };
    }
    let e = err;
    let mut stack: Vec<Rc<dyn StdError>> = Vec::new();
    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Rc<dyn StdError>>, target: Option<Rc<dyn StdError>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct CustomError {
    pub code: Rc<RefCell<Option<i32>>>,
    pub message: Rc<RefCell<Option<String>>>,
}

impl CustomError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.message.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
            message: __go_clone_1_0,
        }
    }
}


impl Default for CustomError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            code: __go_default_0_0,
            message: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl CustomError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Error {}: {}", (*self.code.borrow().as_ref().unwrap()), (*self.message.borrow().as_ref().unwrap())))))
    }
}

impl StdError for CustomError {}


pub fn divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (f64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    if (*b.borrow().as_ref().unwrap()) == 0.0 {
        return (0.0, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("division by zero".to_string()))))));
    }
    ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), Rc::new(RefCell::new(None)))
}

pub fn sqrt(x: Rc<RefCell<Option<f64>>>) -> (f64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    if (*x.borrow().as_ref().unwrap()) < 0.0 {
        return (0.0, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("cannot take square root of negative number: {:.6}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v })))))));
    }

        // Simple approximation
    let mut result = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()) / 2.0)));
    let mut i = Rc::new(RefCell::new(Some(0)));
    while (*i.borrow().as_ref().unwrap()) < 10 {
        { let new_val = ((*result.borrow().as_ref().unwrap()) + (*x.borrow().as_ref().unwrap()) / (*result.borrow().as_ref().unwrap())) / 2.0; *result.borrow_mut() = Some(new_val); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return ((*result.borrow().as_ref().unwrap()), Rc::new(RefCell::new(None)));
}

pub fn process_value(val: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if (*val.borrow().as_ref().unwrap()) < 0 {
        return Rc::new(RefCell::new(Some(Rc::new(CustomError { code: Rc::new(RefCell::new(Some(100))), message: Rc::new(RefCell::new(Some("negative value not allowed".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    }
    if (*val.borrow().as_ref().unwrap()) > 100 {
        return Rc::new(RefCell::new(Some(Rc::new(CustomError { code: Rc::new(RefCell::new(Some(200))), message: Rc::new(RefCell::new(Some("value too large".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    }
    Rc::new(RefCell::new(None))
}

fn main() {
        // Basic error handling
    let (mut result, mut err) = divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(2.0))));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{} {}", format!("{}", "10 / 2 =".to_string()), format!("{}", result));
    }

        // Error case
    { let (__tmp_0, __tmp_1) = divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(0.0)))); result = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        let mut wrapped = Rc::new(RefCell::new(Some(go_error_wrap(format!("wrapped division: {}", match { let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))));
        println!("{} {}", format!("{}", "Wrapped error:".to_string()), format!("{}", { let __text = match wrapped.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{} {}", format!("{}", "Result:".to_string()), format!("{}", result));
    }

        // Formatted error
    let (mut sqrtResult, __tmp_1) = sqrt(Rc::new(RefCell::new(Some(-4.0)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1;;
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Sqrt error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{} {}", format!("{}", "Sqrt result:".to_string()), format!("{}", sqrtResult));
    }

        // Custom error
    { let __rhs_holder = process_value(Rc::new(RefCell::new(Some(-5)))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Process error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }

    { let __rhs_holder = process_value(Rc::new(RefCell::new(Some(150)))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Process error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }

    { let __rhs_holder = process_value(Rc::new(RefCell::new(Some(50)))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Process error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{}", format!("{}", "Value processed successfully".to_string()));
    }

    let mut base = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("stored error".to_string())))));
    let mut stored = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("placeholder".to_string())))));
    { let __rhs_holder = base.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *stored.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", "Stored error:".to_string()), format!("{}", { let __text = match stored.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}