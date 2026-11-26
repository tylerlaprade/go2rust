#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod data;
use data::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    go_install_panic_exit();
    __go_init_all();

        // Access map - transpiler needs to know Users is a map, not a slice
    let mut aliceID = Rc::new(RefCell::new(Some({ let __map_holder = Users.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"alice".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
    print!("Alice's ID: {}\n", { let __v = (*aliceID.borrow().as_ref().unwrap()).clone(); __v });

        // Check map key existence - requires knowing it's a map
    {
        let (mut id, mut ok) = { let __map_holder = Users.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"dave".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false) } };;
        if ok {
            print!("Dave's ID: {}\n", { let __v = (*id.borrow().as_ref().unwrap()).clone(); __v });;
        } else {
            println!("{}", format!("{}", "Dave not found".to_string()));;
        }
    }

        // Access slice - transpiler needs to know Numbers is a slice
    print!("First number: {}\n", (*Numbers.borrow().as_ref().unwrap())[(0) as usize].clone());
    print!("Last number: {}\n", (*Numbers.borrow().as_ref().unwrap())[((({ let __len = (*Numbers.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64) - (1 as i64)) as usize].clone());

        // Access map of slices - complex type resolution
    let mut admins = { let __map_holder = Groups.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"admins".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
    let mut admins = { let __slice_header = GoSliceHeader::go_slice_header(&admins); Rc::new(RefCell::new(__slice_header)) };
    print!("Admin count: {}\n", ({ let __len = (*admins.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }));
    print!("First admin: {}\n", (*admins.borrow().as_ref().unwrap())[(0) as usize].clone());

        // Iterate over map - requires knowing the type
        // Note: map iteration order is non-deterministic, so we'll just count
    let mut count = Rc::new(RefCell::new(Some(0i64)));
    for (_, _) in { let __range_holder = Users.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    print!("User count: {}\n", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v });

        // Access slice of maps - another complex case
    let mut firstRecord = Rc::new(RefCell::new(Some((*Records.borrow().as_ref().unwrap())[(0) as usize].clone())));
    {
        let (mut name, mut ok) = ({
        if let Some(__v) = (*firstRecord.borrow().as_ref().unwrap()).get(&"name".to_string()) {
            let guard = __v.borrow();
            if let Some(ref any_val) = *guard {
                if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                    (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
                } else {
                    (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
                }
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
        }
    });;
        if ok {
            print!("First record name: {}\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v });;
        }
    }

        // Modify map - requires proper type handling
    { let __map_key = "dave".to_string(); let __map_value = Rc::new(RefCell::new(Some(4))); (*Users.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    print!("Dave added with ID: {}\n", { let __map_holder = Users.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"dave".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) });
}

pub(crate) fn __go_init_all() {
    data::__go_zero_globals();
    data::__go_init_order_0();
    data::__go_init_order_1();
    data::__go_init_order_2();
    data::__go_init_order_3();
}
