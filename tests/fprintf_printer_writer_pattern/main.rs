use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone)]
pub struct bytes_Buffer {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl Default for bytes_Buffer {
    fn default() -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())) }
    }
}

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_string())
    }
}

impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_from_string(value: String) -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(value.into_bytes())) }
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_bytes(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_string(&self) -> String {
        String::from_utf8_lossy(&self.__go_data.lock().unwrap()).into_owned()
    }

    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(self.__go_string())))
    }

    pub fn bytes(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(self.__go_bytes())))
    }

    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(self.__go_data.lock().unwrap().len() as i32)))
    }

    pub fn reset(&self) {
        self.__go_data.lock().unwrap().clear();
    }

    pub fn available(&self) -> Rc<RefCell<Option<i32>>> {
        self.len()
    }

    pub fn available_buffer(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn cap(&self) -> Rc<RefCell<Option<i32>>> {
        self.len()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
    }

    pub fn next<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_byte(&self) -> (Rc<RefCell<Option<u8>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<u8>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i64>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_rune(&self) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<String>(String::new()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn truncate<T0>(&self, _arg0: T0) {
        self.reset();
    }

    pub fn unread_byte(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }

    pub fn unread_rune(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
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

    pub fn write_byte<T0: 'static>(&self, arg0: T0) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<u8>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            *v as u8
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<u8>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i32>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default() as u8
        } else {
            0
        };
        self.__go_write_bytes(&[value]);
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }

    pub fn write_rune<T0: 'static>(&self, arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<char>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            char::from_u32(*v as u32).unwrap_or('\0')
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i32>>>>() {
            char::from_u32(v.borrow().as_ref().copied().unwrap_or_default() as u32).unwrap_or('\0')
        } else {
            '\0'
        };
        let mut encoded = [0u8; 4];
        let bytes = value.encode_utf8(&mut encoded).as_bytes().to_vec();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i64>(self.__go_data.lock().unwrap().len() as i64))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
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

    pub fn __go_write_bytes(&self, data: &[u8]) {
        if let Some(buffer) = self.downcast_ref::<bytes_Buffer>() {
            buffer.__go_write_bytes(data);
        }
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
}

impl Default for io_Writer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Rc::new(()) }
    }
}

impl std::fmt::Debug for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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


#[derive(Debug, Clone)]
pub struct printer {
    pub output: Rc<RefCell<Option<io_Writer>>>,
    pub line: Rc<RefCell<Option<i32>>>,
}

impl printer {
    pub fn __go_value_clone(&self) -> Self {
        Self { output: self.output.clone(), line: { let __guard = self.line.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for printer {
    fn default() -> Self {
        Self { output: Rc::new(RefCell::new(None)), line: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for printer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.output.borrow().as_ref().unwrap()), (*self.line.borrow().as_ref().unwrap()))
    }
}


impl printer {
    pub fn write(&mut self, data: Rc<RefCell<Option<Vec<u8>>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut n: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

        let mut m: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
        { let __range_holder = data.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, b) in __range_values.iter().copied().enumerate() {
        if b == ('\n' as u8) {
        { let (__tmp_0, __tmp_1) = (*self.output.borrow().as_ref().unwrap()).write(Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*n.borrow().as_ref().unwrap())) as usize..(i + 1) as usize].to_vec() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *m.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
        { let __rhs = (*m.borrow().as_ref().unwrap()); let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if (*err.borrow()).is_some() {
        return (n, err);
    }
        { let __target = self.line.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else if b == ('#' as u8) {
        { let (__tmp_0, __tmp_1) = { let __s = format!("{:6}  ", (*self.line.borrow().as_ref().unwrap())); let __n = __s.len() as i32; (*self.output.borrow().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (Rc::new(RefCell::new(Some::<i32>(__n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))) }; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
        if (*err.borrow()).is_some() {
        return (n, err);
    }
    }
    } }
        if ((*data.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) > ((*n.borrow().as_ref().unwrap()) as i32) {
        { let (__tmp_0, __tmp_1) = (*self.output.borrow().as_ref().unwrap()).write(Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*n.borrow().as_ref().unwrap())) as usize..].to_vec() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *m.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
        { let __rhs = (*m.borrow().as_ref().unwrap()); let mut guard = n.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return (n, err);
    }
}

fn main() {
    let mut buf: Rc<RefCell<Option<bytes_Buffer>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut p = Rc::new(RefCell::new(Some(printer { output: { let __arg = buf.clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: Option<io_Writer> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Rc::new(RefCell::new(__converted)) }, ..Default::default() })));
    let (mut n, mut err) = (*p.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some(("ab\nc#d\n".to_string()).as_bytes().to_vec()))));
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "err:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
        return;
    }
    println!("{} {} {} {}", format!("{}", "wrote".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", "bytes:".to_string()), format!("{}", (*(*buf.borrow_mut().as_mut().unwrap()).string().borrow().as_ref().unwrap())));
}