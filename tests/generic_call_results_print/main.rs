#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
pub fn local__string(v: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    { let __owned = v.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn local__int(v: Rc<RefCell<Option<i64>>>) -> i64 {
    (*v.borrow().as_ref().unwrap())
}

fn main() {
    ::example_com_genericcallresultsprint_dep::__go_init_all();

    println!("{} {}", format!("{}", (*example_com_genericcallresultsprint_dep::pick::<String>(Rc::new(RefCell::new(Some("qualified".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprint_dep::pick::<String>(Rc::new(RefCell::new(Some("inferred".to_string())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*local__string(Rc::new(RefCell::new(Some("local".to_string())))).borrow().as_ref().unwrap())), format!("{}", local__int(Rc::new(RefCell::new(Some(3))))));
    println!("{} {}", format!("{}", (*example_com_genericcallresultsprint_dep::join::<String, i64>(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(12))), Rc::new(RefCell::new(Some("-".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprint_dep::join::<String, String>(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some("c".to_string()))), Rc::new(RefCell::new(Some("+".to_string())))).borrow().as_ref().unwrap())));
    print!("{}|{}\n", (*example_com_genericcallresultsprint_dep::pick::<String>(Rc::new(RefCell::new(Some("formatted".to_string())))).borrow().as_ref().unwrap()), (*example_com_genericcallresultsprint_dep::join::<i64, i64>(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(":".to_string())))).borrow().as_ref().unwrap()));
}