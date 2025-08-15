mod funcs;
use funcs::*;

use std::sync::{Arc, Mutex};

fn main() {
        // Call function variable - transpiler needs to know ProcessData is a function
    let mut result = (ProcessData.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(5))));
    print!("ProcessData(5) = {}\n", (*result.lock().unwrap().as_mut().unwrap()));

        // Call another function variable
    let mut combined = (CombineStrings.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some("Hello".to_string()))), Arc::new(Mutex::new(Some("World".to_string()))));
    print!("Combined: {}\n", (*combined.lock().unwrap().as_mut().unwrap()));

        // Pass function variable to another function variable
    let mut twice = (ApplyTwice.lock().unwrap().as_ref().unwrap())(ProcessData.clone(), Arc::new(Mutex::new(Some(3))));
    print!("ApplyTwice(ProcessData, 3) = {}\n", (*twice.lock().unwrap().as_mut().unwrap()));

        // Call no-parameter function variable
    let mut greeting = (GetGreeting.lock().unwrap().as_ref().unwrap())();
    println!("{}", (*greeting.lock().unwrap().as_mut().unwrap()));

        // Call function with multiple returns
    let (mut q, mut r) = (DivMod.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(17))), Arc::new(Mutex::new(Some(5))));
    print!("17 / 5 = {} remainder {}\n", (*q.lock().unwrap().as_mut().unwrap()), (*r.lock().unwrap().as_mut().unwrap()));

        // Compare with regular function call
    let mut regular = regular_double(Arc::new(Mutex::new(Some(5))));
    print!("RegularDouble(5) = {}\n", (*regular.lock().unwrap().as_mut().unwrap()));

        // Use function returned by function
    let mut triple = make_multiplier(Arc::new(Mutex::new(Some(3))));
    print!("Triple(4) = {}\n", (*(triple.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(4)))).lock().unwrap().as_ref().unwrap()));

        // Call dynamically assigned function
    let mut dynamic = (DynamicFunc.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some("test".to_string()))));
    println!("{}", (*dynamic.lock().unwrap().as_mut().unwrap()));

        // Assign function variable to local variable
    let mut localFunc = Arc::new(Mutex::new(Some(PROCESS_DATA)));
    print!("Local func call: {}\n", (*(localFunc.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(7)))).lock().unwrap().as_ref().unwrap()));

        // Function variable in conditional
    let mut conditionalFunc: Arc<Mutex<Option<Box<dyn Fn(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> + Send + Sync>>>>;
    if (*result.lock().unwrap().as_mut().unwrap()) > 0 {
        { let new_val = PROCESS_DATA; *conditionalFunc.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = REGULAR_DOUBLE; *conditionalFunc.lock().unwrap() = Some(new_val); };
    }
    print!("Conditional func(6) = {}\n", (*(conditionalFunc.lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some(6)))).lock().unwrap().as_ref().unwrap()));
}