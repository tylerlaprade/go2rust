use std::cell::{RefCell};
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


impl From<bytes_Buffer> for io_Writer {
    fn from(_value: bytes_Buffer) -> Self {
        Self::default()
    }
}


pub fn r#use(w: Rc<RefCell<Option<io_Writer>>>) {
    let _ = (*w.borrow().as_ref().unwrap());
}

pub fn make_buffer() -> Rc<RefCell<Option<bytes_Buffer>>> {

    let (mut stdout, mut stderr) = (Rc::new(RefCell::new(Some(bytes_Buffer { ..Default::default() }))), Rc::new(RefCell::new(Some(bytes_Buffer { ..Default::default() }))));
    r#use({ let __arg = stdout.clone(); let __converted = { let __arg_guard = __arg.borrow(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Rc::new(RefCell::new(Some(__converted))) });
    r#use({ let __arg = stderr.clone(); let __converted = { let __arg_guard = __arg.borrow(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Rc::new(RefCell::new(Some(__converted))) });
    return stdout.clone();
}

fn main() {
    if (*make_buffer().borrow()).is_some() {
        println!("{}", "buffer".to_string());
    }
}