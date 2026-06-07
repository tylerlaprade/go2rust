use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct printer {
    pub output: Arc<Mutex<Option<io_Writer>>>,
    pub line: Arc<Mutex<Option<i32>>>,
}

impl printer {
    pub fn __go_value_clone(&self) -> Self {
        Self { output: self.output.clone(), line: { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for printer {
    fn default() -> Self {
        Self { output: Arc::new(Mutex::new(None)), line: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for printer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.output.lock().unwrap().as_ref().unwrap()), (*self.line.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for printer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl printer {
    pub fn write(&mut self, data: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut m: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, b) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = b; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = (*self.output.lock().unwrap().as_ref().unwrap()).write(Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *m.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        { let __rhs = (*m.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
        { let __target = self.line.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else if { let __tmp_x = b; let __tmp_y = ('#' as i32) as u8; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = { let __s = format!("{:6}  ", (*self.line.lock().unwrap().as_ref().unwrap())); let __n = __s.len() as i32; (*self.output.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }
    } }
        if { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x > __tmp_y } {
        { let (__tmp_0, __tmp_1) = (*self.output.lock().unwrap().as_ref().unwrap()).write(Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *m.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        { let __rhs = (*m.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
}

fn main() {
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut buf: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut p = Arc::new(Mutex::new(Some(printer { output: Arc::new(Mutex::new(Some({ let __writer = buf.clone(); io_Writer::__go_from_with_write(__writer.clone(), move |__data| { let mut __guard = __writer.lock().unwrap(); if let Some(__target) = __guard.as_mut() { let _ = __target.write(Arc::new(Mutex::new(Some(__data.to_vec())))); } }) }))), ..Default::default() })));
    let (mut n, mut err) = { let __recv = p.clone(); let __recv_ptr: *mut printer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut printer }; let __result = unsafe { &mut *__recv_ptr }.write(Arc::new(Mutex::new(Some(("ab\nc#d\n".to_string()).as_bytes().to_vec())))); __result };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "err:".to_string()), format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));
        return;
    }
    println!("{} {} {} {}", format!("{}", "wrote".to_string()), format!("{}", n), format!("{}", "bytes:".to_string()), format!("{}", (*(*buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for printer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
