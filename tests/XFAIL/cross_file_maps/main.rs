mod data;
use data::*;

use std::sync::{Arc, Mutex};

fn main() {
    let mut aliceID = Arc::new(Mutex::new(Some((*(*Users.lock().unwrap().as_ref().unwrap()).get(&"alice".to_string()).unwrap().lock().unwrap().as_ref().unwrap()))));
    print!("Alice's ID: {}\n", (*aliceID.lock().unwrap().as_mut().unwrap()));

    let (mut id, mut ok) = match (*Users.lock().unwrap().as_ref().unwrap()).get(&"dave".to_string()) { Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false)))) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("Dave's ID: {}\n", (*id.lock().unwrap().as_mut().unwrap()));
    } else {
        println!("{}", "Dave not found".to_string());
    }

    print!("First number: {}\n", (*NUMBERS.lock().unwrap().as_ref().unwrap())[0 as usize].clone());
    print!("Last number: {}\n", (*NUMBERS.lock().unwrap().as_ref().unwrap())[(*NUMBERS.lock().unwrap().as_ref().unwrap()).len() - 1 as usize].clone());

    let mut admins = Arc::new(Mutex::new(Some((*(*Groups.lock().unwrap().as_ref().unwrap()).get(&"admins".to_string()).unwrap().lock().unwrap().as_ref().unwrap()))));
    print!("Admin count: {}\n", (*admins.lock().unwrap().as_ref().unwrap()).len());
    print!("First admin: {}\n", (*admins.lock().unwrap().as_ref().unwrap())[0 as usize].clone());

    let mut count = Arc::new(Mutex::new(Some(0)));
    for (_, _) in (*Users.lock().unwrap().as_ref().unwrap()).clone() {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    print!("User count: {}\n", (*count.lock().unwrap().as_mut().unwrap()));

    let mut firstRecord = Arc::new(Mutex::new(Some((*RECORDS.lock().unwrap().as_ref().unwrap())[0 as usize].clone())));
    let (mut name, mut ok) = ({
        let val = (*(*firstRecord.lock().unwrap().as_ref().unwrap()).get(&"name".to_string()).unwrap().lock().unwrap().as_ref().unwrap()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))
            } else {
                (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
            }
        } else {
            (Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(false))))
        }
    });
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("First record name: {}\n", (*name.lock().unwrap().as_mut().unwrap()));
    }

    (*USERS.lock().unwrap().as_mut().unwrap())["dave".to_string()] = 4;
    print!("Dave added with ID: {}\n", (*(*Users.lock().unwrap().as_ref().unwrap()).get(&"dave".to_string()).unwrap().lock().unwrap().as_ref().unwrap()));
}