use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Object;

impl std::fmt::Display for types_Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Object>")
    }
}


impl types_Object {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_TypeName;

impl std::fmt::Display for types_TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_TypeName>")
    }
}


impl types_TypeName {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub fn count_type_names(objs: Rc<RefCell<Option<Vec<types_Object>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut count = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = objs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for o in __range_values.iter() {
        let (_, mut ok) = ({
        let val = o.clone();
        if let Some(typed_val) = val.downcast_ref::<types_TypeName>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
    return count.clone();
}

fn main() {
    if false {
        println!("{}", (*count_type_names(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}