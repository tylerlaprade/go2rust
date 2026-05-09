use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_FileInfo;

impl std::fmt::Display for fs_FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_FileInfo>")
    }
}


impl fs_FileInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn is_dir(&self) -> Rc<RefCell<Option<bool>>> {
        Rc::new(RefCell::new(Some::<bool>(Default::default())))
    }
}


pub mod os {
    use super::*;
    pub fn stat<T0>(_arg0: T0) -> (Rc<RefCell<Option<fs_FileInfo>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<fs_FileInfo>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


fn main() {
    if false {
        let (mut info, mut err) = os::stat(".".to_string());
        if (*err.borrow()).is_none() {
        println!("{}", (*(*info.borrow().as_ref().unwrap()).is_dir().borrow().as_ref().unwrap()));
    }
    }
}