use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bytes_Buffer;

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bytes_Buffer>")
    }
}


impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct io_Writer;

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}


impl io_Writer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
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


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub out: Rc<RefCell<Option<io_Writer>>>,
    pub err: Rc<RefCell<Option<io_Writer>>>,
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.out.borrow().as_ref().unwrap()), (*self.err.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut h: Rc<RefCell<Option<holder>>> = Rc::new(RefCell::new(Some(Default::default())));
    let (_, mut ok) = ({
        let val = (*h.borrow().as_ref().unwrap()).out.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<os_File>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<os_File>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<os_File>)), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
        println!("{}", "file".to_string());
    } else {
        println!("{}", "not file".to_string());
    }
    let (mut buf, _) = ({
        let val = (*h.borrow().as_ref().unwrap()).err.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<bytes_Buffer>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<bytes_Buffer>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<bytes_Buffer>)), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*buf.borrow()).is_some() {
        println!("{}", "buffer".to_string());
    } else {
        println!("{}", "not buffer".to_string());
    }
}