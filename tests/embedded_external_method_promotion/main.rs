use std::cell::{RefCell};
use std::error::Error as StdError;
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
    pub fn available(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(Default::default())))
    }
    pub fn available_buffer(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Default::default())))
    }
    pub fn bytes(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Default::default())))
    }
    pub fn cap(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(Default::default())))
    }
    pub fn grow<T0>(&self, _arg0: T0) {
    }
    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(Default::default())))
    }
    pub fn next<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Default::default())))
    }
    pub fn read<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn read_byte(&self) -> (Rc<RefCell<Option<u8>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<u8>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<Vec<u8>>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn read_from<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i64>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn read_rune(&self) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn read_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<String>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn reset(&self) {
    }
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(Default::default())))
    }
    pub fn truncate<T0>(&self, _arg0: T0) {
    }
    pub fn unread_byte(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
    pub fn unread_rune(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
    pub fn write<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn write_byte<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }
    pub fn write_rune<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn write_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
    pub fn write_to<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i64>(Default::default()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


#[derive(Debug, Clone, Default)]
pub struct writer {
    pub buffer: Rc<RefCell<Option<bytes_Buffer>>>,
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


impl writer {
    pub fn available(&self) -> Rc<RefCell<Option<i32>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.available()
    }

    pub fn available_buffer(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.available_buffer()
    }

    pub fn bytes(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.bytes()
    }

    pub fn cap(&self) -> Rc<RefCell<Option<i32>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cap()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.grow(_arg0)
    }

    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn next<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.next(_arg0)
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read(_arg0)
    }

    pub fn read_byte(&self) -> (Rc<RefCell<Option<u8>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_byte()
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_bytes(_arg0)
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_from(_arg0)
    }

    pub fn read_rune(&self) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_rune()
    }

    pub fn read_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_string(_arg0)
    }

    pub fn reset(&self) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.reset()
    }

    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.string()
    }

    pub fn truncate<T0>(&self, _arg0: T0) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.truncate(_arg0)
    }

    pub fn unread_byte(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.unread_byte()
    }

    pub fn unread_rune(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.unread_rune()
    }

    pub fn write<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write(_arg0)
    }

    pub fn write_byte<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_byte(_arg0)
    }

    pub fn write_rune<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_rune(_arg0)
    }

    pub fn write_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_string(_arg0)
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_to(_arg0)
    }
}

fn main() {
    let mut w = Rc::new(RefCell::new(Some(writer { buffer: Rc::new(RefCell::new(Some(bytes_Buffer { ..Default::default() }))), ..Default::default() })));
    (*w.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("go".to_string()))));
    (*w.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some(("rust".to_string()).as_bytes().to_vec()))));
    (*w.borrow_mut().as_mut().unwrap()).write_byte(Rc::new(RefCell::new(Some(('!' as i32) as u8))));
    (*w.borrow_mut().as_mut().unwrap()).reset();
    let mut length = (*w.borrow_mut().as_mut().unwrap()).len();
    let mut text = (*w.borrow_mut().as_mut().unwrap()).string();
    print!("len={} string={:?}\n", { let __v = (*length.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*text.borrow().as_ref().unwrap()).clone(); __v });
}