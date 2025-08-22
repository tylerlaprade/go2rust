mod data;
use data::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Access map - transpiler needs to know Users is a map, not a slice
    let mut aliceID = Rc::new(RefCell::new(Some((*(*Users.borrow().as_ref().unwrap()).get(&"alice".to_string()).unwrap().borrow().as_ref().unwrap()))));
    print!("Alice's ID: {}\n", (*aliceID.borrow_mut().as_mut().unwrap()));

        // Check map key existence - requires knowing it's a map
    let (mut id, mut ok) = match (*Users.borrow().as_ref().unwrap()).get(&"dave".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("Dave's ID: {}\n", (*id.borrow_mut().as_mut().unwrap()));
    } else {
        println!("{}", "Dave not found".to_string());
    }

        // Access slice - transpiler needs to know Numbers is a slice
    print!("First number: {}\n", (*NUMBERS.borrow().as_ref().unwrap())[0 as usize].clone());
    print!("Last number: {}\n", (*NUMBERS.borrow().as_ref().unwrap())[(*NUMBERS.borrow().as_ref().unwrap()).len() - 1 as usize].clone());

        // Access map of slices - complex type resolution
    let mut admins = Rc::new(RefCell::new(Some((*(*Groups.borrow().as_ref().unwrap()).get(&"admins".to_string()).unwrap().borrow().as_ref().unwrap()))));
    print!("Admin count: {}\n", (*admins.borrow().as_ref().unwrap()).len());
    print!("First admin: {}\n", (*admins.borrow().as_ref().unwrap())[0 as usize].clone());

        // Iterate over map - requires knowing the type
        // Note: map iteration order is non-deterministic, so we'll just count
    let mut count = Rc::new(RefCell::new(Some(0)));
    for (_, _) in (*Users.borrow().as_ref().unwrap()).clone() {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    print!("User count: {}\n", (*count.borrow_mut().as_mut().unwrap()));

        // Access slice of maps - another complex case
    let mut firstRecord = Rc::new(RefCell::new(Some((*RECORDS.borrow().as_ref().unwrap())[0 as usize].clone())));
    let (mut name, mut ok) = ({
        let val = (*(*firstRecord.borrow().as_ref().unwrap()).get(&"name".to_string()).unwrap().borrow().as_ref().unwrap()).clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("First record name: {}\n", (*name.borrow_mut().as_mut().unwrap()));
    }

        // Modify map - requires proper type handling
    (*Users.borrow_mut().as_mut().unwrap()).insert("dave".to_string(), Rc::new(RefCell::new(Some(4))));
    print!("Dave added with ID: {}\n", (*(*Users.borrow().as_ref().unwrap()).get(&"dave".to_string()).unwrap().borrow().as_ref().unwrap()));
}