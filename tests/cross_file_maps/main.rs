mod data;
use data::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    data::__go_init_all();

        // Access map - transpiler needs to know Users is a map, not a slice
    let mut aliceID = Rc::new(RefCell::new(Some((*Users.borrow().as_ref().unwrap()).get(&"alice".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0))));
    print!("Alice's ID: {}\n", { let __v = (*aliceID.borrow().as_ref().unwrap()).clone(); __v });

        // Check map key existence - requires knowing it's a map
    let (mut id, mut ok) = match (*Users.borrow().as_ref().unwrap()).get(&"dave".to_string()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false)))) };
    if (*ok.borrow().as_ref().unwrap()) {
        print!("Dave's ID: {}\n", { let __v = (*id.borrow().as_ref().unwrap()).clone(); __v });
    } else {
        println!("{}", "Dave not found".to_string());
    }

        // Access slice - transpiler needs to know Numbers is a slice
    print!("First number: {}\n", (*Numbers.borrow().as_ref().unwrap())[(0) as usize].clone());
    print!("Last number: {}\n", (*Numbers.borrow().as_ref().unwrap())[(((*Numbers.borrow().as_ref().unwrap()).len() as i32) - (1 as i32)) as usize].clone());

        // Access map of slices - complex type resolution
    let mut admins = Rc::new(RefCell::new(Some((*Groups.borrow().as_ref().unwrap()).get(&"admins".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| vec![]))));
    print!("Admin count: {}\n", (*admins.borrow().as_ref().unwrap()).len());
    print!("First admin: {}\n", (*admins.borrow().as_ref().unwrap())[(0) as usize].clone());

        // Iterate over map - requires knowing the type
        // Note: map iteration order is non-deterministic, so we'll just count
    let mut count = Rc::new(RefCell::new(Some(0)));
    for (_, _) in { let __range_holder = Users.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    print!("User count: {}\n", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v });

        // Access slice of maps - another complex case
    let mut firstRecord = Rc::new(RefCell::new(Some((*Records.borrow().as_ref().unwrap())[(0) as usize].clone())));
    let (mut name, mut ok) = ({
        if let Some(__v) = (*firstRecord.borrow().as_ref().unwrap()).get(&"name".to_string()) {
            let guard = __v.borrow();
            if let Some(ref any_val) = *guard {
                if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                    (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
                } else {
                    (Rc::new(RefCell::new(Some(std::string::String::new()))), Rc::new(RefCell::new(Some(false))))
                }
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        print!("First record name: {}\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v });
    }

        // Modify map - requires proper type handling
    { let __map_key = "dave".to_string(); let __map_value = Rc::new(RefCell::new(Some(4))); (*Users.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    print!("Dave added with ID: {}\n", (*Users.borrow().as_ref().unwrap()).get(&"dave".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0));
}