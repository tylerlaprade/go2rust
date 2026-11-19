use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    ::example_com_stringkey_dep::__go_init_all();

    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::new())));
    { let __map_key = example_com_stringkey_dep::NAME.to_string(); let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map_holder = seen.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&example_com_stringkey_dep::NAME.to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false) }));
}