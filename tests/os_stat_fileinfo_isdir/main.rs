use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct fs_FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: i64,
}

impl std::fmt::Display for fs_FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fs_FileInfo>")
    }
}


impl fs_FileInfo {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(self.name.clone())))
    }
    pub fn size(&self) -> Rc<RefCell<Option<i64>>> {
        Rc::new(RefCell::new(Some::<i64>(self.size)))
    }
    pub fn is_dir(&self) -> Rc<RefCell<Option<bool>>> {
        Rc::new(RefCell::new(Some::<bool>(self.is_dir)))
    }
}


pub mod os {
    use super::*;
    use std::path::Path;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Rc<RefCell<Option<String>>> {
        fn into_go_string(self) -> String {
            self.borrow().as_ref().cloned().unwrap_or_default()
        }
    }

    type GoError = Rc<RefCell<Option<Box<dyn StdError>>>>;

    fn no_error() -> GoError {
        Rc::new(RefCell::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Rc::new(RefCell::new(Some(Box::new(err))))
    }

    pub fn stat<T0: GoStringArg>(_arg0: T0) -> (Rc<RefCell<Option<fs_FileInfo>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::metadata(&path) {
            Ok(metadata) => {
                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
                (Rc::new(RefCell::new(Some::<fs_FileInfo>(fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }))), no_error())
            }
            Err(err) => (Rc::new(RefCell::new(Some::<fs_FileInfo>(fs_FileInfo::default()))), io_error(err)),
        }
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