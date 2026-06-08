use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    debuglog_off::{DLOG_ENABLED, put_cached_dlogger},
    mem_darwin::{sys_alloc_o_s},
    mgc::{itoa_div},
    panic::{throw},
    print::{gwrite, hex, printlock, printunlock},
    proc::{runtimeInitTime},
    runtime2::{eface, eface_of},
    slice::{slice},
    string::{slicebytetostringtmp, stringStruct},
    stubs::{noescape},
    symtab::{findfunc, firstmoduledata, funcInfo, funcline, funcname, moduledata},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG_LOG_BYTES: i32 = 16 << 10;


pub(crate) const DEBUG_LOG_STRING_LIMIT: i32 = DEBUG_LOG_BYTES / 8;


pub(crate) const DEBUG_LOG_UNKNOWN: i32 = 1 + 0;
pub(crate) const DEBUG_LOG_BOOL_TRUE: i32 = 1 + 1;
pub(crate) const DEBUG_LOG_BOOL_FALSE: i32 = 1 + 2;
pub(crate) const DEBUG_LOG_INT: i32 = 1 + 3;
pub(crate) const DEBUG_LOG_UINT: i32 = 1 + 4;
pub(crate) const DEBUG_LOG_HEX: i32 = 1 + 5;
pub(crate) const DEBUG_LOG_PTR: i32 = 1 + 6;
pub(crate) const DEBUG_LOG_STRING: i32 = 1 + 7;
pub(crate) const DEBUG_LOG_CONST_STRING: i32 = 1 + 8;
pub(crate) const DEBUG_LOG_STRING_OVERFLOW: i32 = 1 + 9;
pub(crate) const DEBUG_LOG_P_C: i32 = 1 + 10;
pub(crate) const DEBUG_LOG_TRACEBACK: i32 = 1 + 11;


pub(crate) const DEBUG_LOG_HEADER_SIZE: i32 = 2;
pub(crate) const DEBUG_LOG_SYNC_SIZE: i32 = DEBUG_LOG_HEADER_SIZE + 2 * 8;


/// A dloggerImpl writes to the debug log.
///
/// To obtain a dloggerImpl, call dlog(). When done with the dloggerImpl, call
/// end().
#[derive(Clone)]
pub struct dloggerImpl {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub w: Arc<Mutex<Option<debugLogWriter>>>,
    pub all_link: Arc<Mutex<Option<dloggerImpl>>>,
    pub owned: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
}

impl dloggerImpl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.all_link.clone();
        let __go_clone_3_0 = { let __guard = self.owned.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            w: __go_clone_1_0,
            all_link: __go_clone_2_0,
            owned: __go_clone_3_0,
        }
    }
}


impl Default for dloggerImpl {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(debugLogWriter::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            w: __go_default_1_0,
            all_link: __go_default_2_0,
            owned: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for dloggerImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.all_link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.owned.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for dloggerImpl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A debugLogWriter is a ring buffer of binary debug log records.
///
/// A log record consists of a 2-byte framing header and a sequence of
/// fields. The framing header gives the size of the record as a little
/// endian 16-bit value. Each field starts with a byte indicating its
/// type, followed by type-specific data. If the size in the framing
/// header is 0, it's a sync record consisting of two little endian
/// 64-bit values giving a new time base.
///
/// Because this is a ring buffer, new records will eventually
/// overwrite old records. Hence, it maintains a reader that consumes
/// the log as it gets overwritten. That reader state is where an
/// actual log reader would start.
#[derive(Clone)]
pub struct debugLogWriter {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub write: Arc<Mutex<Option<u64>>>,
    pub data: Arc<Mutex<Option<debugLogBuf>>>,
    pub tick: Arc<Mutex<Option<u64>>>,
    pub nano: Arc<Mutex<Option<u64>>>,
    pub r: Arc<Mutex<Option<debugLogReader>>>,
    pub buf: Arc<Mutex<Option<[u8; 10]>>>,
}

impl debugLogWriter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.write.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_1 = { let __guard = self.nano.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            write: __go_clone_1_0,
            data: __go_clone_2_0,
            tick: __go_clone_3_0,
            nano: __go_clone_3_1,
            r: __go_clone_4_0,
            buf: __go_clone_5_0,
        }
    }
}


impl Default for debugLogWriter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(debugLogBuf::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(debugLogReader::default())));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            __blank_0_0: __go_default_0_0,
            write: __go_default_1_0,
            data: __go_default_2_0,
            tick: __go_default_3_0,
            nano: __go_default_3_1,
            r: __go_default_4_0,
            buf: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for debugLogWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.write.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.tick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.nano.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.r.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", format_slice(&self.buf));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}

impl GoJsonDecode for debugLogWriter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct debugLogBuf {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub b: Arc<Mutex<Option<[u8; 16384]>>>,
}

impl debugLogBuf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            b: __go_clone_1_0,
        }
    }
}


impl Default for debugLogBuf {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            __blank_0_0: __go_default_0_0,
            b: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for debugLogBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.b));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for debugLogBuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct debugLogReader {
    pub data: Arc<Mutex<Option<debugLogBuf>>>,
    pub begin: Arc<Mutex<Option<u64>>>,
    pub end: Arc<Mutex<Option<u64>>>,
    pub tick: Arc<Mutex<Option<u64>>>,
    pub nano: Arc<Mutex<Option<u64>>>,
}

impl debugLogReader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.data.clone();
        let __go_clone_1_0 = { let __guard = self.begin.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_1 = { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_1 = { let __guard = self.nano.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            data: __go_clone_0_0,
            begin: __go_clone_1_0,
            end: __go_clone_1_1,
            tick: __go_clone_2_0,
            nano: __go_clone_2_1,
        }
    }
}


impl Default for debugLogReader {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            data: __go_default_0_0,
            begin: __go_default_1_0,
            end: __go_default_1_1,
            tick: __go_default_2_0,
            nano: __go_default_2_1,
        }
    }
}

impl std::fmt::Display for debugLogReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.data.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.begin.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.end.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.tick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.nano.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for debugLogReader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static allDloggers: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<dloggerImpl>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *allDloggers.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_zero_globals() {
    *allDloggers.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


impl dloggerImpl {
    ///go:nosplit
    pub fn end(&mut self) {
                // Fill in framing header.
        let mut size = Arc::new(Mutex::new(Some({
            let __tmp_x = (*(*self.w.lock().unwrap().as_ref().unwrap()).write.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*(*self.w.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).end.lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        })));
        if !(*self.w.lock().unwrap().as_mut().unwrap()).write_frame_at(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*self.w.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        throw(Arc::new(Mutex::new(Some("record too large".to_string()))));
    }
                // Commit the record.
        { let new_val = { let __selector_holder = (*self.w.lock().unwrap().as_ref().unwrap()).write.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*(*self.w.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).end.lock().unwrap() = Some(new_val); };
                // Attempt to return this logger to the cache.
        if put_cached_dlogger(Arc::new(Mutex::new(Some(self.clone())))) {
        return;
    }
                // Return the logger to the global pool.
        (*self.owned.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
    }

    ///go:nosplit
    pub fn b(&self, x: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        if { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_BOOL_TRUE as u8))));
    } else {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_BOOL_FALSE as u8))));
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn i(&self, x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.i64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i64))))
    }

    ///go:nosplit
    pub fn i8(&self, x: Arc<Mutex<Option<i8>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.i64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i64))))
    }

    ///go:nosplit
    pub fn i16(&self, x: Arc<Mutex<Option<i16>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.i64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i64))))
    }

    ///go:nosplit
    pub fn i32(&self, x: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.i64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i64))))
    }

    ///go:nosplit
    pub fn i64(&self, x: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_INT as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn u(&self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.u64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    ///go:nosplit
    pub fn uptr(&self, x: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.u64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    ///go:nosplit
    pub fn u8(&self, x: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.u64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    ///go:nosplit
    pub fn u16(&self, x: Arc<Mutex<Option<u16>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.u64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    ///go:nosplit
    pub fn u32(&self, x: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        self.u64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    ///go:nosplit
    pub fn u64(&self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_UINT as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn hex(&self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_HEX as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn p(&self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_PTR as u8))));
        if { let __nil_result = (*x.lock().unwrap()).is_none(); __nil_result } {
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some(0 as u64))));
    } else {
        let mut v: GoPtr<crate::runtime2::eface> = eface_of(x.clone());
        { let _switch_val = {
    let __tmp_x = { let __selector_holder = { let __ptr_value = { let __ptr_value = v.with_mut(|__ptr_value| __ptr_value._type.clone()); __ptr_value }.with_mut(|__ptr_value| __ptr_value.kind_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8))));
    __tmp_x & __tmp_y
};
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::CHAN as u8))))) || _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::FUNC as u8))))) || _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::MAP as u8))))) || _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::POINTER as u8))))) || _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::UNSAFE_POINTER as u8))))) {
            (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize as u64))));
        } else {
            throw(Arc::new(Mutex::new(Some("not a pointer type".to_string()))));
        }
    }
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn s(&self, x: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        let mut strData = { let __go_unsafe_result: Arc<Mutex<Option<u8>>> = unimplemented!("unsafe.StringData requires unsafe intrinsic support"); __go_unsafe_result };
        let mut datap = firstmoduledata.clone();
        if {
            let __go_cond_0 = {
                let __go_cond_1 = { let __tmp_x = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 4; __tmp_x > __tmp_y };
                if __go_cond_1 {
                    let __go_cond_2 = {
                        let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).etext.clone(); __field }.lock().unwrap().as_ref().unwrap());
                        let __tmp_y = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&strData) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
                        __tmp_x <= __tmp_y
                    };
                    __go_cond_2
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_3 = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&strData) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).end.clone(); __field }.lock().unwrap().as_ref().unwrap());
                    __tmp_x < __tmp_y
                };
                __go_cond_3
            } else {
                false
            }
        } {
                // String constants are in the rodata section, which
                // isn't recorded in moduledata. But it has to be
                // somewhere between etext and end.
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_CONST_STRING as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()).len() as u64))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some(({
            let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&strData) as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).etext.clone(); __field }.lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        }) as u64))));
    } else {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_STRING as u8))));
                // We can't use unsafe.Slice as it may panic, which isn't safe
                // in this (potentially) nowritebarrier context.
        let mut b: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
        let mut bb: GoPtr<crate::slice::slice> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&b.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&strData) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = bb.with_mut(|__ptr_value| __ptr_value.array.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
        {
            let __tmp_0 = (*x.lock().unwrap().as_ref().unwrap()).len();
            let __tmp_1 = (*x.lock().unwrap().as_ref().unwrap()).len();
            *{ let __ptr_value = bb.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.lock().unwrap() = Some(__tmp_0 as i32);
            *{ let __ptr_value = bb.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.lock().unwrap() = Some(__tmp_1 as i32);
        };
        if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2048; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (DEBUG_LOG_STRING_LIMIT) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    }
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))));
        (*self.w.lock().unwrap().as_mut().unwrap()).bytes(b.clone());
        if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x != __tmp_y } {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_STRING_OVERFLOW as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some(({ let __tmp_x = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x - __tmp_y }) as u64))));
    }
    }
                // String constants are in the rodata section, which
                // isn't recorded in moduledata. But it has to be
                // somewhere between etext and end.
                // We can't use unsafe.Slice as it may panic, which isn't safe
                // in this (potentially) nowritebarrier context.
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn pc(&self, x: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_P_C as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))));
        Arc::new(Mutex::new(Some(self.clone())))
    }

    ///go:nosplit
    pub fn traceback(&self, x: Arc<Mutex<Option<Vec<usize>>>>) -> Arc<Mutex<Option<dloggerImpl>>> {
        (*self.w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(DEBUG_LOG_TRACEBACK as u8))));
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))));
        { let __range_holder = x.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pc in __range_values.iter().copied() {
        (*self.w.lock().unwrap().as_mut().unwrap()).uvarint(Arc::new(Mutex::new(Some(pc as u64))));
    } }
        Arc::new(Mutex::new(Some(self.clone())))
    }
}

impl debugLogWriter {
    ///go:nosplit
    pub fn ensure(&self, n: Arc<Mutex<Option<u64>>>) {
        while {
            let __tmp_x = { let __tmp_x = (*self.write.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
            let __tmp_y = {
                let __tmp_x = (*(*self.r.lock().unwrap().as_ref().unwrap()).begin.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
                __tmp_x + __tmp_y
            };
            __tmp_x >= __tmp_y
        } {
                // Consume record at begin.
        if {
            let __tmp_x = (*self.r.lock().unwrap().as_mut().unwrap()).skip();
            let __tmp_y = !(0 as u64) as u64;
            __tmp_x == __tmp_y
        } {
                // Wrapped around within a record.
                //
                // TODO(austin): It would be better to just
                // eat the whole buffer at this point, but we
                // have to communicate that to the reader
                // somehow.
        throw(Arc::new(Mutex::new(Some("record wrapped around".to_string()))));
    }
    }
    }

    ///go:nosplit
    pub fn write_frame_at(&mut self, pos: Arc<Mutex<Option<u64>>>, size: Arc<Mutex<Option<u64>>>) -> bool {
        (*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_mut().unwrap())[({
            let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize] = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_mut().unwrap())[({
            let __tmp_x = ({ let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y });
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        return { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFFFF as u64; __tmp_x <= __tmp_y };
    }

    ///go:nosplit
    pub fn write_sync(&mut self, tick: Arc<Mutex<Option<u64>>>, nano: Arc<Mutex<Option<u64>>>) {
        {
            let __tmp_0 = (*tick.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*nano.lock().unwrap().as_ref().unwrap()).clone();
            *self.tick.lock().unwrap() = Some(__tmp_0);
            *self.nano.lock().unwrap() = Some(__tmp_1);
        };
        self.ensure(Arc::new(Mutex::new(Some(DEBUG_LOG_HEADER_SIZE as u64))));
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.write.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some(0 as u64))); self.write_frame_at(__method_arg0, __method_arg1) };
        { let __target = self.write.clone(); let __rhs = DEBUG_LOG_HEADER_SIZE as u64; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        self.write_uint64_l_e(Arc::new(Mutex::new(Some({ let __arg_holder = tick.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.write_uint64_l_e(Arc::new(Mutex::new(Some({ let __arg_holder = nano.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __selector_holder = self.write.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.r.lock().unwrap().as_ref().unwrap()).end.lock().unwrap() = Some(new_val); };
    }

    ///go:nosplit
    pub fn write_uint64_l_e(&mut self, x: Arc<Mutex<Option<u64>>>) {
        let mut b: Arc<Mutex<Option<[u8; 8]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        (*b.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(1) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(2) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(3) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 24; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(4) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(5) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 40; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(6) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 48; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        (*b.lock().unwrap().as_mut().unwrap())[(7) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 56; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        self.bytes(Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
    }

    ///go:nosplit
    pub fn byte(&mut self, x: Arc<Mutex<Option<u8>>>) {
        self.ensure(Arc::new(Mutex::new(Some(1 as u64))));
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.write.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let __target = self.write.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        (*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_mut().unwrap())[({
            let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize] = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    ///go:nosplit
    pub fn bytes(&mut self, mut x: Arc<Mutex<Option<Vec<u8>>>>) {
        self.ensure(Arc::new(Mutex::new(Some((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))));
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = self.write.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let __target = self.write.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        while { let __tmp_x = ((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut n = {
            let _dst_start = ({
                let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v };
                let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
                __tmp_x % __tmp_y
            }) as usize;
            let _dst_len = (*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = { let __copy_src_holder = x.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
        { let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = pos.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); x = new_val; };
    }
    }

    ///go:nosplit
    pub fn varint(&mut self, x: Arc<Mutex<Option<i64>>>) {
        let mut u: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = ({ let __tmp_x = !(*x.lock().unwrap().as_ref().unwrap()) as u64; let __tmp_y = 1; __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x | __tmp_y }; *u.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }); *u.lock().unwrap() = Some(new_val); };
    }
                // complement i, bit 0 is 1
                // do not complement i, bit 0 is 0
        self.uvarint(Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    ///go:nosplit
    pub fn uvarint(&mut self, mut u: Arc<Mutex<Option<u64>>>) {
        let mut i = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u64; __tmp_x >= __tmp_y } {
        (*self.buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = (*Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0x80 as u8; __tmp_x | __tmp_y };
        { let __rhs = 7 as u64; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        (*self.buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.bytes(__method_arg0) };
    }
}

impl debugLogReader {
    ///go:nosplit
    pub fn skip(&mut self) -> u64 {
                // Read size at pos.
        if {
            let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y };
            let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        return !0 as u64;
    }
        let mut size = Arc::new(Mutex::new(Some({ let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.begin.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.read_uint16_l_e_at(__method_arg0) } as u64)));
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                // Sync packet.
        { let new_val = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y }))); self.read_uint64_l_e_at(__method_arg0) }; *self.tick.lock().unwrap() = Some(new_val); };
        { let new_val = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y }; let __tmp_y = 8 as u64; __tmp_x + __tmp_y }))); self.read_uint64_l_e_at(__method_arg0) }; *self.nano.lock().unwrap() = Some(new_val); };
        { let new_val = DEBUG_LOG_SYNC_SIZE as u64; *size.lock().unwrap() = Some(new_val); };
    }
                // Sync packet.
        if {
            let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
            let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        return !0 as u64;
    }
        { let __target = self.begin.clone(); let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    ///go:nosplit
    pub fn read_uint16_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u16 {
        return {
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
                let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v };
                let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
                __tmp_x % __tmp_y
            }) as usize].clone() } as u16))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = {
                let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
                    let __tmp_x = ({ let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y });
                    let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
                    __tmp_x % __tmp_y
                }) as usize].clone() } as u16))).lock().unwrap().as_ref().unwrap());
                let __tmp_y = 8;
                __tmp_x << __tmp_y
            };
            __tmp_x | __tmp_y
        };
    }

    ///go:nosplit
    pub fn read_uint64_l_e_at(&self, mut pos: Arc<Mutex<Option<u64>>>) -> u64 {
        let mut b: Arc<Mutex<Option<[u8; 8]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        for i in 0..(({ let __range_holder = b.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*b.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __seq = { let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize].clone() };
        { let mut guard = pos.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return {
            let __go_binary_0 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_1 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_2 = 8;
            let __go_binary_3 = __go_binary_1 << __go_binary_2;
            let __go_binary_4 = __go_binary_0 | __go_binary_3;
            let __go_binary_5 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_6 = 16;
            let __go_binary_7 = __go_binary_5 << __go_binary_6;
            let __go_binary_8 = __go_binary_4 | __go_binary_7;
            let __go_binary_9 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_10 = 24;
            let __go_binary_11 = __go_binary_9 << __go_binary_10;
            let __go_binary_12 = __go_binary_8 | __go_binary_11;
            let __go_binary_13 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_14 = 32;
            let __go_binary_15 = __go_binary_13 << __go_binary_14;
            let __go_binary_16 = __go_binary_12 | __go_binary_15;
            let __go_binary_17 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_18 = 40;
            let __go_binary_19 = __go_binary_17 << __go_binary_18;
            let __go_binary_20 = __go_binary_16 | __go_binary_19;
            let __go_binary_21 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_22 = 48;
            let __go_binary_23 = __go_binary_21 << __go_binary_22;
            let __go_binary_24 = __go_binary_20 | __go_binary_23;
            let __go_binary_25 = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap());
            let __go_binary_26 = 56;
            let __go_binary_27 = __go_binary_25 << __go_binary_26;
            let __go_binary_28 = __go_binary_24 | __go_binary_27;
            __go_binary_28
        };
    }

    pub fn peek(&mut self) -> u64 {
    let mut tick: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

                // Consume any sync records.
        let mut size = Arc::new(Mutex::new(Some(0 as u64)));
        while { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        if {
            let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y };
            let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        return !0 as u64;
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.begin.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.read_uint16_l_e_at(__method_arg0) } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        break
    }
        if {
            let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_SYNC_SIZE as u64; __tmp_x + __tmp_y };
            let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        return !0 as u64;
    }

                // Sync packet.
        { let new_val = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y }))); self.read_uint64_l_e_at(__method_arg0) }; *self.tick.lock().unwrap() = Some(new_val); };
        { let new_val = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y }; let __tmp_y = 8 as u64; __tmp_x + __tmp_y }))); self.read_uint64_l_e_at(__method_arg0) }; *self.nano.lock().unwrap() = Some(new_val); };
        { let __target = self.begin.clone(); let __rhs = DEBUG_LOG_SYNC_SIZE as u64; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Sync packet.
                // Peek tick delta.
        if {
            let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
            let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        return !0 as u64;
    }
        let mut pos = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = DEBUG_LOG_HEADER_SIZE as u64; __tmp_x + __tmp_y })));
        let mut u: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut i = Arc::new(Mutex::new(Some(0 as u64)));
    loop {
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize].clone() })));
        { let mut guard = pos.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & ! __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        break
    }
        { let __rhs = 7 as u64; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if { let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x > __tmp_y } {
        return !0 as u64;
    }
        return { let __tmp_x = (*self.tick.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
    }

    pub fn header(&mut self) -> (u64, u64, u64, i32) {
    let mut end: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut tick: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut nano: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut p: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

                // Read size. We've already skipped sync packets and checked
                // bounds in peek.
        let mut size = Arc::new(Mutex::new(Some({ let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.begin.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.read_uint16_l_e_at(__method_arg0) } as u64)));
        { let new_val = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *end.lock().unwrap() = Some(new_val); };
        { let __target = self.begin.clone(); let __rhs = DEBUG_LOG_HEADER_SIZE as u64; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Read tick, nano, and p.
        { let new_val = { let __tmp_x = self.uvarint(); let __tmp_y = (*self.tick.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *tick.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = self.uvarint(); let __tmp_y = (*self.nano.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *nano.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some(self.varint() as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_val; };
        return ((*end.lock().unwrap().as_ref().unwrap()), (*tick.lock().unwrap().as_ref().unwrap()), (*nano.lock().unwrap().as_ref().unwrap()), (*p.lock().unwrap().as_ref().unwrap()));
    }

    pub fn uvarint(&mut self) -> u64 {
        let mut u: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut i = Arc::new(Mutex::new(Some(0 as u64)));
    loop {
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize].clone() })));
        { let __target = self.begin.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & ! __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        break
    }
        { let __rhs = 7 as u64; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    pub fn varint(&mut self) -> i64 {
        let mut u = self.uvarint();
        let mut v: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __tmp_x = u; let __tmp_y = 1 as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = u; let __tmp_y = 1; __tmp_x >> __tmp_y }) as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = !({ let __tmp_x = u; let __tmp_y = 1; __tmp_x >> __tmp_y }) as i64; *v.lock().unwrap() = Some(new_val); };
    }
        return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    pub fn print_val(&mut self) -> bool {
        let mut typ = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({
            let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize].clone() })));
        { let __target = self.begin.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        '__go_switch_1: loop {
        { let _switch_val = { let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (DEBUG_LOG_UNKNOWN as u8) {
            {
            let __go_print_arg_0 = format!("{}", "<unknown kind>".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_BOOL_TRUE as u8) {
            {
            let __go_print_arg_0 = format!("{}", true);
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_BOOL_FALSE as u8) {
            {
            let __go_print_arg_0 = format!("{}", false);
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_INT as u8) {
            {
            let __go_print_arg_0 = format!("{}", self.varint());
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_UINT as u8) {
            {
            let __go_print_arg_0 = format!("{}", self.uvarint());
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_HEX as u8) || _switch_val == (DEBUG_LOG_PTR as u8) {
            {
            let __go_print_arg_0 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(self.uvarint() as u64)))));
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_STRING as u8) {
            let mut sl = self.uvarint();
            if {
                let __tmp_x = { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = sl; __tmp_x + __tmp_y };
                let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap());
                __tmp_x > __tmp_y
            } {
        { let new_val = { let __selector_holder = self.end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.begin.lock().unwrap() = Some(new_val); };
        {
            let __go_print_arg_0 = format!("{}", "<string length corrupted>".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        break '__go_switch_1
    }
            while { let __tmp_x = sl; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        let mut b = Arc::new(Mutex::new(Some({ let __seq_holder = (*self.data.lock().unwrap().as_ref().unwrap()).b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({
            let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.data.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64;
            __tmp_x % __tmp_y
        }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = sl; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (sl) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); b = new_val; };
    }
        { let __target = self.begin.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u64))).lock().unwrap().as_ref().unwrap()); sl = sl - __rhs; };
        gwrite(b.clone());
    }
        } else if _switch_val == (DEBUG_LOG_CONST_STRING as u8) {
            let (mut len, mut ptr) = (Arc::new(Mutex::new(Some(self.uvarint() as i32))), Arc::new(Mutex::new(Some(self.uvarint() as usize))));
            { let __rhs = (*{ let __field = (*firstmoduledata.lock().unwrap().as_ref().unwrap()).etext.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = ptr.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                        // We can't use unsafe.String as it may panic, which isn't safe
                        // in this (potentially) nowritebarrier context.
            let mut str = Arc::new(Mutex::new(Some(stringStruct { str: Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap())))), len: Arc::new(Mutex::new(Some({ let __arg_holder = len.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
            let mut s = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&str.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<String>(unimplemented!("unsafe.Pointer conversion to String")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
            {
            let __go_print_arg_0 = format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v });
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (DEBUG_LOG_STRING_OVERFLOW as u8) {
            {
            let __go_print_arg_0 = format!("{}", "..(".to_string());
            let __go_print_arg_1 = format!("{}", self.uvarint());
            let __go_print_arg_2 = format!("{}", " more bytes)..".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        } else if _switch_val == (DEBUG_LOG_P_C as u8) {
            print_debug_log_p_c(Arc::new(Mutex::new(Some(self.uvarint() as usize))), Arc::new(Mutex::new(Some(false))));
        } else if _switch_val == (DEBUG_LOG_TRACEBACK as u8) {
            let mut n = Arc::new(Mutex::new(Some(self.uvarint() as i32)));
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "\n\t".to_string());
            eprint!("{}", __go_print_arg_0)
        };

                // gentraceback PCs are always return PCs.
                // Convert them to call PCs.
                //
                // TODO(austin): Expand inlined frames.
        print_debug_log_p_c(Arc::new(Mutex::new(Some(self.uvarint() as usize))), Arc::new(Mutex::new(Some(true))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        } else {
            {
            let __go_print_arg_0 = format!("{}", "<unknown field type ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*typ.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " pos ".to_string());
            let __go_print_arg_3 = format!("{}", { let __tmp_x = (*self.begin.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x - __tmp_y });
            let __go_print_arg_4 = format!("{}", " end ".to_string());
            let __go_print_arg_5 = format!("{}", (*self.end.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", ">\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
            return false;
        }
    };
        break;
    }
                // We can't use unsafe.String as it may panic, which isn't safe
                // in this (potentially) nowritebarrier context.
                // gentraceback PCs are always return PCs.
                // Convert them to call PCs.
                //
                // TODO(austin): Expand inlined frames.
        true
    }
}

/// printDebugLog prints the debug log.
pub fn print_debug_log() {
    if DLOG_ENABLED {
        print_debug_log_impl();
    }
}

pub fn print_debug_log_impl() {
        // This function should not panic or throw since it is used in
        // the fatal panic path and this may deadlock.
    printlock();

        // Get the list of all debug logs.
    let mut allp_local: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(allDloggers.clone())))) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut all: GoPtr<dloggerImpl> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(internal_runtime_atomic::loaduintptr({ let __go_ptr = allp_local.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });

        // Count the logs.
    let mut n = Arc::new(Mutex::new(Some(0)));
    let mut l: GoPtr<dloggerImpl> = all.clone();
    while !l.is_nil() {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        l = GoPtr::local({ let __ptr_value = l.borrow(); let __field_value = __ptr_value.as_ref().unwrap().all_link.clone(); __field_value });
    }
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        printunlock();
        return;
    }

        // Prepare read state for all logs.
    type readState = AnonymousStruct2;

        // Use sysAllocOS instead of sysAlloc because we want to interfere
        // with the runtime as little as possible, and sysAlloc updates accounting.
    let mut state1 = sys_alloc_o_s(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<readState>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))));
    if { let __nil_result = (*state1.lock().unwrap()).is_none(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "failed to allocate read state for".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "logs".to_string());
            eprintln!("{} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        printunlock();
        return;
    }
    let mut state = Arc::new(Mutex::new(Some({ let __seq_holder = Arc::new(Mutex::new({ let __ptr = state1.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[readState; 1048576]>(unimplemented!("unsafe.Pointer conversion to [readState; 1048576]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    {
        let mut l: GoPtr<dloggerImpl> = all.clone();
        for i in 0..(({ let __range_holder = state.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut s: Option<GoSliceElemPtr<readState>> = Some(GoSliceElemPtr::new(state.clone(), (i) as usize));
        { let new_val = { let __selector_holder = (*{ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.w.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap() = Some(new_val); };
        { let new_val = true; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).first.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*(*{ let __ptr_value = l.with_mut(|__ptr_value| __ptr_value.w.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).begin.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).lost.lock().unwrap() = Some(new_val); };
        { let new_val = { let __promoted_recv = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.peek(); __result }; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).next_tick.lock().unwrap() = Some(new_val); };
        l = GoPtr::local({ let __ptr_value = l.borrow(); let __field_value = __ptr_value.as_ref().unwrap().all_link.clone(); __field_value });
    }
    }

        // Print records.
    loop {
                // Find the next record.
        let mut best: Arc<Mutex<Option<AnonymousStruct3>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = !(0 as u64) as u64; *(*best.lock().unwrap().as_ref().unwrap()).tick.lock().unwrap() = Some(new_val); };
        for i in 0..(({ let __range_holder = state.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if {
            let __tmp_x = (*{ let __seq = { let __seq_holder = state.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.next_tick.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __field = (*best.lock().unwrap().as_ref().unwrap()).tick.clone(); __field }.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        } {
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = state.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.next_tick.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*best.lock().unwrap().as_ref().unwrap()).tick.lock().unwrap() = Some(new_val); };
        { let new_val = i as i32; *(*best.lock().unwrap().as_ref().unwrap()).i.lock().unwrap() = Some(new_val); };
    }
    }
        if { let __tmp_x = (*{ let __field = (*best.lock().unwrap().as_ref().unwrap()).tick.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
        break
    }

                // Print record.
        let mut s: Option<GoSliceElemPtr<readState>> = Some(GoSliceElemPtr::new(state.clone(), ((*{ let __field = (*best.lock().unwrap().as_ref().unwrap()).i.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize));
        if (*{ let __field = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).first.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
            let __go_print_arg_0 = format!("{}", ">> begin log ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*best.lock().unwrap().as_ref().unwrap()).i.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
        if { let __tmp_x = (*{ let __field = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).lost.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "; lost first ".to_string());
            let __go_print_arg_1 = format!("{}", { let __tmp_x = (*{ let __field = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).lost.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 10; __tmp_x >> __tmp_y });
            let __go_print_arg_2 = format!("{}", "KB".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", " <<\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = false; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).first.lock().unwrap() = Some(new_val); };
    }

        let (mut end, _, mut nano, mut p) = { let __promoted_recv = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.header(); __result };
        let mut oldEnd = Arc::new(Mutex::new(Some({ let __selector_holder = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap().as_ref().unwrap().end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = end; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap().as_ref().unwrap().end.lock().unwrap() = Some(new_val); };

        {
            let __go_print_arg_0 = format!("{}", "[".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        let mut tmpbuf_local: Arc<Mutex<Option<[u8; 21]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let mut pnano = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(nano as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*runtimeInitTime.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*pnano.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // Logged before runtimeInitTime was set.
        { let new_val = 0 as i64; *pnano.lock().unwrap() = Some(new_val); };
    }
                // Logged before runtimeInitTime was set.
        let mut pnanoBytes = itoa_div(Arc::new(Mutex::new(Some({ let __seq_holder = tmpbuf_local.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some((*pnano.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some(9))));
        {
            let __go_print_arg_0 = format!("{}", (*slicebytetostringtmp(GoPtr::raw({ let __ptr = noescape(Arc::new(Mutex::new(Some({ let __seq_holder = pnanoBytes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(Some((*pnanoBytes.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))).lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
        {
            let __go_print_arg_0 = format!("{}", " P ".to_string());
            let __go_print_arg_1 = format!("{}", p);
            let __go_print_arg_2 = format!("{}", "] ".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };

        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = (*{ let __field = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap().as_ref().unwrap().begin.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap().as_ref().unwrap().end.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", " ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        if !{ let __promoted_recv = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.print_val(); __result } {
                // Abort this P log.
        {
            let __go_print_arg_0 = format!("{}", "<aborting P log>".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = (*oldEnd.lock().unwrap().as_ref().unwrap()); end = new_val; };
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Abort this P log.
        eprintln!();

                // Move on to the next record.
        { let new_val = end; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap().as_ref().unwrap().begin.lock().unwrap() = Some(new_val); };
        { let new_val = oldEnd.lock().unwrap().as_ref().unwrap().clone(); *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.lock().unwrap().as_ref().unwrap().end.lock().unwrap() = Some(new_val); };
        { let new_val = { let __promoted_recv = (*s.as_ref().unwrap().borrow().as_ref().unwrap()).debug_log_reader.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.peek(); __result }; *(*s.as_ref().unwrap().borrow().as_ref().unwrap()).next_tick.lock().unwrap() = Some(new_val); };
    }

        // Find the next record.
        // Print record.
        // Logged before runtimeInitTime was set.
        // Abort this P log.
        // Move on to the next record.
    printunlock();
}

/// printDebugLogPC prints a single symbolized PC. If returnPC is true,
/// pc is a return PC that must first be converted to a call PC.
pub fn print_debug_log_p_c(mut pc: Arc<Mutex<Option<usize>>>, returnPC: Arc<Mutex<Option<bool>>>) {
    let mut r#fn = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __v = (*returnPC.lock().unwrap().as_ref().unwrap()).clone(); __v } && (!(*r#fn.lock().unwrap().as_ref().unwrap()).valid() || { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*r#fn.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x > __tmp_y }) {
                // TODO(austin): Don't back up if the previous frame
                // was a sigpanic.
        { let mut guard = pc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        // TODO(austin): Don't back up if the previous frame
        // was a sigpanic.
    {
            let __go_print_arg_0 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*pc.lock().unwrap().as_ref().unwrap()) as u64)))));
            eprint!("{}", __go_print_arg_0)
        };
    if !(*r#fn.lock().unwrap().as_ref().unwrap()).valid() {
        {
            let __go_print_arg_0 = format!("{}", " [unknown PC]".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        let mut name = funcname(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let (mut file, mut line) = funcline(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
            let __go_print_arg_0 = format!("{}", " [".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "+".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*r#fn.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x - __tmp_y } as u64)))));
            let __go_print_arg_4 = format!("{}", " ".to_string());
            let __go_print_arg_5 = format!("{}", { let __v = (*file.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_6 = format!("{}", ":".to_string());
            let __go_print_arg_7 = format!("{}", line);
            let __go_print_arg_8 = format!("{}", "]".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    }
}

#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub debug_log_reader: Arc<Mutex<Option<debugLogReader>>>,
    pub first: Arc<Mutex<Option<bool>>>,
    pub lost: Arc<Mutex<Option<u64>>>,
    pub next_tick: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.debug_log_reader.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.first.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.lost.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.next_tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            debug_log_reader: __go_clone_0_0,
            first: __go_clone_1_0,
            lost: __go_clone_2_0,
            next_tick: __go_clone_3_0,
        }
    }
}

impl AnonymousStruct2 {
    pub fn header(&mut self) -> (u64, u64, u64, i32) {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.header()
    }

    pub fn peek(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.peek()
    }

    pub fn print_val(&mut self) -> bool {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.print_val()
    }

    pub fn read_uint16_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u16 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint16_l_e_at(pos)
    }

    pub fn read_uint64_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint64_l_e_at(pos)
    }

    pub fn skip(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.skip()
    }

    pub fn uvarint(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.uvarint()
    }

    pub fn varint(&mut self) -> i64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint()
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(debugLogReader::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            debug_log_reader: __go_default_0_0,
            first: __go_default_1_0,
            lost: __go_default_2_0,
            next_tick: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.debug_log_reader.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.first.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.lost.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.next_tick.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub tick: Arc<Mutex<Option<u64>>>,
    pub i: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            tick: __go_clone_0_0,
            i: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            tick: __go_default_0_0,
            i: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.tick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.i.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for dloggerImpl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for debugLogWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for debugLogBuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for debugLogReader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
