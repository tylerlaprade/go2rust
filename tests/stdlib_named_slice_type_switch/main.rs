use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct scanner_Error {
    pub msg: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for scanner_Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<scanner_Error>")
    }
}

impl std::error::Error for scanner_Error {}


impl scanner_Error {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(Default::default())))
    }
}


#[derive(Debug, Clone, Default)]
pub struct scanner_ErrorList(pub Rc<RefCell<Option<Vec<Rc<RefCell<Option<scanner_Error>>>>>>>);
impl std::fmt::Display for scanner_ErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<scanner_ErrorList>")
    }
}

impl std::error::Error for scanner_ErrorList {}


impl scanner_ErrorList {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(Default::default())))
    }
}


pub fn describe(mut err: Rc<RefCell<Option<Box<dyn StdError>>>>) {
    {
    let _ts_subject = err.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if _ts_val.and_then(|__v| __v.downcast_ref::<scanner_ErrorList>()).is_some() {
        let err = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<scanner_ErrorList>()).unwrap().clone())));
        { let __range_holder = { let __named_slice = (*err.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        println!("{}", format!("{}", (*(*item.borrow().as_ref().unwrap()).msg.borrow().as_ref().unwrap()).clone()));
    } };
    } else {
        let err = err.clone();
        println!("{}", format!("{}", "other".to_string()));;
    }
    }
}

fn main() {
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));
    describe(err.clone());
}