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
        panic!("exec_ExitError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
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
        if (*ok.borrow().as_ref().unwrap()) {
            println!("{}", format!("{}", (*(*ee.borrow().as_ref().unwrap()).stderr.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));;
        }
    }
    }
}