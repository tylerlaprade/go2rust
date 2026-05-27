use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};


struct GoFile {
    file: Option<std::fs::File>,
}

impl GoFile {
    fn create(path: &str) -> Result<Self, std::io::Error> {
        std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .map(|file| GoFile { file: Some(file) })
    }

    fn empty() -> Self {
        GoFile { file: None }
    }

    fn write_string(&mut self, text: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn std::error::Error>>>>) {
        let text = (*text.borrow().as_ref().unwrap()).clone();
        match self.file.as_mut() {
            Some(file) => match std::io::Write::write_all(file, text.as_bytes()) {
                Ok(()) => (Rc::new(RefCell::new(Some(text.len() as i32))), Rc::new(RefCell::new(None))),
                Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from(e))))),
            },
            None => (
                Rc::new(RefCell::new(Some(0))),
                Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from(std::io::Error::new(std::io::ErrorKind::Other, "invalid file"))))),
            ),
        }
    }

    fn close(&mut self) -> Rc<RefCell<Option<Box<dyn std::error::Error>>>> {
        self.file = None;
        Rc::new(RefCell::new(None))
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

    pub fn __go_read_all_for_copy(&self) -> Vec<u8> {
        while self.__go_wait_for_close && !self.__go_closed.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        self.__go_read_all()
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
}


fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        { let __path = "test.txt".to_string(); match std::fs::remove_file(&__path) { Ok(()) => Rc::new(RefCell::new(None::<Box<dyn StdError>>)), Err(e) => Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(e)))) } };
    }));

    let (mut file, mut err) = { let __path = "test.txt".to_string(); match GoFile::create(&__path) { Ok(file) => (Rc::new(RefCell::new(Some(file))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))), Err(e) => (Rc::new(RefCell::new(Some(GoFile::empty()))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(e))))) } };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
    let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        (*file_defer_captured.borrow_mut().as_mut().unwrap()).close();
    }));

    (*file.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("Hello, World!".to_string()))));
    println!("{}", format!("{}", "File written successfully".to_string()));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}