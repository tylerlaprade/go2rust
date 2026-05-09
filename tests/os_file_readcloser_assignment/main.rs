use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct io_ReadCloser;

impl std::fmt::Display for io_ReadCloser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ReadCloser>")
    }
}


impl io_ReadCloser {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn close(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct os_File;

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}


impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<os_File> for io_ReadCloser {
    fn from(_value: os_File) -> Self {
        Self::default()
    }
}


pub mod os {
    use super::*;
    pub fn open<T0>(_arg0: T0) -> (Rc<RefCell<Option<os_File>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<os_File>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


fn main() {
    let __go_os_args = Rc::new(RefCell::new(Some(std::env::args().collect::<Vec<String>>())));

    let (mut file, mut err) = os::open((*__go_os_args.clone().borrow().as_ref().unwrap())[(0) as usize].clone());
    if (*err.borrow()).is_some() {
        panic!("{:?}", (*err.borrow().as_ref().unwrap()));
    }

    let mut rc: Rc<RefCell<Option<io_ReadCloser>>> = Rc::new(RefCell::new(None));
    { let new_val = { let __arg = file.clone(); let __arg_guard = __arg.borrow(); (*__arg_guard.as_ref().unwrap()).clone().into() }; *rc.borrow_mut() = Some(new_val); };
    let mut err = (*rc.borrow().as_ref().unwrap()).close();
    if (*err.borrow()).is_some() {
        panic!("{:?}", (*err.borrow().as_ref().unwrap()));
    }
    eprintln!("{}", "closed".to_string());
}