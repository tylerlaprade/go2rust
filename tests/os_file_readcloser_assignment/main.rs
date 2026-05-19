use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct io_ReadCloser {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl io_ReadCloser {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn close(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
}

impl Default for io_ReadCloser {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for io_ReadCloser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ReadCloser>")
    }
}

impl std::fmt::Display for io_ReadCloser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ReadCloser>")
    }
}

impl PartialEq for io_ReadCloser {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_ReadCloser {}

impl PartialOrd for io_ReadCloser {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_ReadCloser {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
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
        Self::__go_from(_value)
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