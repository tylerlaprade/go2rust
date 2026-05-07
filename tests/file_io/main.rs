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

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct os_File;

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}


impl os_File {
    pub fn close(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
    pub fn write_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    __defer_stack.push(Box::new(move || {
        { let __path = "test.txt".to_string(); match std::fs::remove_file(&__path) { Ok(()) => Rc::new(RefCell::new(None::<Box<dyn StdError>>)), Err(e) => Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(e)))) } };
    }));

    let (mut file, mut err) = { let __path = "test.txt".to_string(); match GoFile::create(&__path) { Ok(file) => (Rc::new(RefCell::new(Some(file))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))), Err(e) => (Rc::new(RefCell::new(Some(GoFile::empty()))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(e))))) } };
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.borrow().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let file_defer_captured = file.clone(); __defer_stack.push(Box::new(move || {
        (*file_defer_captured.borrow_mut().as_mut().unwrap()).close();
    }));

    (*file.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("Hello, World!".to_string()))));
    println!("{}", "File written successfully".to_string());

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}