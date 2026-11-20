use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = Rc::new(RefCell::new(Some({ let __s = "1, 2,x, 30".to_string(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut field in __range_values.iter().cloned() {
        { let new_val = (*Rc::new(RefCell::new(Some({ let __s = field.clone(); __s.trim().to_string() }))).borrow().as_ref().unwrap()).clone(); field = new_val; };
        let (mut n, mut err) = { let __atoi_input = field.clone(); match __atoi_input.parse::<i32>() { Ok(n) => (n, Rc::new(RefCell::new(None))), Err(_) => (0 as i32, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input))))))) } };
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {} {}", format!("{}", "skip".to_string()), format!("{}", field), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        continue
    }
        { let __rhs = n; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        println!("{} {} {}", format!("{}", field), format!("{}", "->".to_string()), format!("{}", n));
    } }
    for word in &vec!["true".to_string(), "nope".to_string()] {
        let (mut b, mut err) = { let __parse_bool_input = word.clone(); match __parse_bool_input.as_str() { "1" | "t" | "T" | "TRUE" | "true" | "True" => (true, Rc::new(RefCell::new(None))), "0" | "f" | "F" | "FALSE" | "false" | "False" => (false, Rc::new(RefCell::new(None))), _ => (false, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("strconv.ParseBool: parsing \"{}\": invalid syntax", __parse_bool_input))))))) } };
        println!("{} {} {}", format!("{}", word), format!("{}", b), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    }
    println!("{}", format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));
}