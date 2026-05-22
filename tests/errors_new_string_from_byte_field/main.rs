use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct exec_ExitError {
    pub stderr: Rc<RefCell<Option<Vec<u8>>>>,
}

impl std::fmt::Display for exec_ExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<exec_ExitError>")
    }
}

impl std::error::Error for exec_ExitError {}


impl exec_ExitError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(Default::default())))
    }
}


fn main() {
    if false {
        let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));
        {
        let (mut ee, mut ok) = ({
        let val = err.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<exec_ExitError>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) && ((*(*ee.borrow().as_ref().unwrap()).stderr.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) > (0 as i32) {
            { let __rhs_holder = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from((*Rc::new(RefCell::new(Some(String::from_utf8({ let __slice_holder = (*ee.borrow().as_ref().unwrap()).stderr.clone(); let __slice_guard = __slice_holder.borrow(); (*__slice_guard.as_ref().unwrap()).clone() }).unwrap()))).borrow().as_ref().unwrap()).clone())))).clone(); let new_val = { let mut guard = __rhs_holder.borrow_mut(); guard.take() }; *err.borrow_mut() = new_val; };;
            println!("{}", format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));;
        }
    }
    }
}