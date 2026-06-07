use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};


thread_local! {
    static __GO_OS_ARGS: std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Some(std::env::args().collect::<Vec<String>>())));
}

fn go_os_args() -> std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> {
    __GO_OS_ARGS.with(|args| args.clone())
}

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
        if let Some(file) = self.downcast_ref::<os_File>() {
            return file.close();
        }
        panic!("io_ReadCloser.close bridge: unsupported concrete receiver; transpile io/os source instead - see AGENTS.md and docs/bridge_debt.md#io-readcloser-close-dispatch")
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


#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
        }
    }
}

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}

impl PartialEq for os_File {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__go_data, &other.__go_data)
    }
}

impl Eq for os_File {}

impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn close(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<String>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i64>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let data = self.__go_read_all();
        let mut n = 0i32;
        if offset >= 0 {
            let start = offset as usize;
            if start < data.len() {
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
                    let mut guard = v.borrow_mut();
                    if let Some(target) = guard.as_mut() {
                        let count = std::cmp::min(target.len(), data.len() - start);
                        target[..count].copy_from_slice(&data[start..start + count]);
                        n = count as i32;
                    }
                }
            }
        }
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


impl From<os_File> for io_ReadCloser {
    fn from(_value: os_File) -> Self {
        Self::__go_from(_value)
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

    type GoError = Rc<RefCell<Option<Box<dyn std::error::Error>>>>;

    fn no_error() -> GoError {
        Rc::new(RefCell::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        Rc::new(RefCell::new(Some(Box::new(err))))
    }

    pub fn open<T0: GoStringArg>(_arg0: T0) -> (Rc<RefCell<Option<os_File>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let path = _arg0.into_go_string();
        match std::fs::read(&path) {
            Ok(data) => {
                let file = os_File { __go_data: std::sync::Arc::new(std::sync::Mutex::new(data)), __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), __go_wait_for_close: false };
                (Rc::new(RefCell::new(Some::<os_File>(file))), no_error())
            }
            Err(err) => (Rc::new(RefCell::new(None::<os_File>)), io_error(err)),
        }
    }
}


fn main() {
    let (mut file, mut err) = os::open((*go_os_args().borrow().as_ref().unwrap())[(0) as usize].clone());
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        panic!("{}", (*err.borrow().as_ref().unwrap()));
    }

    let mut rc: Rc<RefCell<Option<io_ReadCloser>>> = Rc::new(RefCell::new(None));
    { let new_val = { let __arg = file.clone(); let __arg_guard = __arg.borrow(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(io_ReadCloser::default) }; *rc.borrow_mut() = Some(new_val); };
    {
        let mut err = (*rc.borrow().as_ref().unwrap()).close();;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            panic!("{}", (*err.borrow().as_ref().unwrap()));;
        }
    }
    eprintln!("{}", format!("{}", "closed".to_string()));
}