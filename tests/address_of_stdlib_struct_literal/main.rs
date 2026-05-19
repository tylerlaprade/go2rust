use std::cell::{RefCell};
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



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


#[derive(Clone)]
pub struct io_Writer {
    pub __go_id: usize,
    pub __go_value: Rc<dyn std::any::Any>,
}

impl io_Writer {
    pub fn __go_from<T: 'static>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Rc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for io_Writer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl PartialEq for io_Writer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Writer {}

impl PartialOrd for io_Writer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Writer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


impl From<bytes_Buffer> for io_Writer {
    fn from(_value: bytes_Buffer) -> Self {
        Self::__go_from(_value)
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
        println!("{}", format!("{}", "buffer".to_string()));
    }
}