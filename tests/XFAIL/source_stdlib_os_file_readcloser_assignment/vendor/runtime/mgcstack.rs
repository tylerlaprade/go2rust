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
    mgcwork::{__WORKBUF_SIZE, getempty, putempty, workbuf, workbufhdr},
    panic::{throw},
    runtime2::{stack},
    stack::{stackObjectRecord},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const STACK_TRACE_DEBUG: bool = false;


/// Buffer for pointers found during stack tracing.
/// Must be smaller than or equal to workbuf.
#[derive(Clone)]
pub struct stackWorkBuf {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub stack_work_buf_hdr: Arc<Mutex<Option<stackWorkBufHdr>>>,
    pub obj: Arc<Mutex<Option<[usize; 252]>>>,
}

impl stackWorkBuf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.stack_work_buf_hdr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.obj.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            stack_work_buf_hdr: __go_clone_1_0,
            obj: __go_clone_2_0,
        }
    }
}


impl Default for stackWorkBuf {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(stackWorkBufHdr::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            __blank_0_0: __go_default_0_0,
            stack_work_buf_hdr: __go_default_1_0,
            obj: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for stackWorkBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.stack_work_buf_hdr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.obj));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for stackWorkBuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Header declaration must come after the buf declaration above, because of issue #14620.
#[derive(Clone)]
pub struct stackWorkBufHdr {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub workbufhdr: Arc<Mutex<Option<workbufhdr>>>,
    pub next: Arc<Mutex<Option<stackWorkBuf>>>,
}

impl stackWorkBufHdr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.workbufhdr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.next.clone();
        Self {
            __blank_0_0: __go_clone_0_0,
            workbufhdr: __go_clone_1_0,
            next: __go_clone_2_0,
        }
    }
}


impl Default for stackWorkBufHdr {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(workbufhdr::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            __blank_0_0: __go_default_0_0,
            workbufhdr: __go_default_1_0,
            next: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for stackWorkBufHdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.workbufhdr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for stackWorkBufHdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Buffer for stack objects found on a goroutine stack.
/// Must be smaller than or equal to workbuf.
#[derive(Clone)]
pub struct stackObjectBuf {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub stack_object_buf_hdr: Arc<Mutex<Option<stackObjectBufHdr>>>,
    pub obj: Arc<Mutex<Option<[stackObject; 63]>>>,
}

impl stackObjectBuf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.stack_object_buf_hdr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.obj.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            stack_object_buf_hdr: __go_clone_1_0,
            obj: __go_clone_2_0,
        }
    }
}


impl Default for stackObjectBuf {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(stackObjectBufHdr::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        Self {
            __blank_0_0: __go_default_0_0,
            stack_object_buf_hdr: __go_default_1_0,
            obj: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for stackObjectBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.stack_object_buf_hdr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.obj));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for stackObjectBuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct stackObjectBufHdr {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub workbufhdr: Arc<Mutex<Option<workbufhdr>>>,
    pub next: GoPtr<stackObjectBuf>,
}

impl stackObjectBufHdr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.workbufhdr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.next.clone();
        Self {
            __blank_0_0: __go_clone_0_0,
            workbufhdr: __go_clone_1_0,
            next: __go_clone_2_0,
        }
    }
}


impl Default for stackObjectBufHdr {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(workbufhdr::default())));
        let __go_default_2_0 = GoPtr::nil();
        Self {
            __blank_0_0: __go_default_0_0,
            workbufhdr: __go_default_1_0,
            next: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for stackObjectBufHdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.workbufhdr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { if self.next.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for stackObjectBufHdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A stackObject represents a variable on the stack that has had
/// its address taken.
#[derive(Clone)]
pub struct stackObject {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub off: Arc<Mutex<Option<u32>>>,
    pub size: Arc<Mutex<Option<u32>>>,
    pub r: Arc<Mutex<Option<stackObjectRecord>>>,
    pub left: GoPtr<stackObject>,
    pub right: GoPtr<stackObject>,
}

impl stackObject {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.r.clone();
        let __go_clone_4_0 = self.left.clone();
        let __go_clone_5_0 = self.right.clone();
        Self {
            __blank_0_0: __go_clone_0_0,
            off: __go_clone_1_0,
            size: __go_clone_2_0,
            r: __go_clone_3_0,
            left: __go_clone_4_0,
            right: __go_clone_5_0,
        }
    }
}


impl Default for stackObject {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = GoPtr::nil();
        let __go_default_5_0 = GoPtr::nil();
        Self {
            __blank_0_0: __go_default_0_0,
            off: __go_default_1_0,
            size: __go_default_2_0,
            r: __go_default_3_0,
            left: __go_default_4_0,
            right: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for stackObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.off.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { let __guard = self.r.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", { if self.left.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_5 = format!("{}", { if self.right.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}

impl GoJsonDecode for stackObject {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A stackScanState keeps track of the state used during the GC walk
/// of a goroutine.
#[derive(Clone)]
pub struct stackScanState {
    pub stack: Arc<Mutex<Option<stack>>>,
    pub conservative: Arc<Mutex<Option<bool>>>,
    pub buf: Arc<Mutex<Option<stackWorkBuf>>>,
    pub free_buf: Arc<Mutex<Option<stackWorkBuf>>>,
    pub cbuf: Arc<Mutex<Option<stackWorkBuf>>>,
    pub head: GoPtr<stackObjectBuf>,
    pub tail: GoPtr<stackObjectBuf>,
    pub nobjs: Arc<Mutex<Option<i32>>>,
    pub root: GoPtr<stackObject>,
}

impl stackScanState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.conservative.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.buf.clone();
        let __go_clone_3_0 = self.free_buf.clone();
        let __go_clone_4_0 = self.cbuf.clone();
        let __go_clone_5_0 = self.head.clone();
        let __go_clone_6_0 = self.tail.clone();
        let __go_clone_7_0 = { let __guard = self.nobjs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = self.root.clone();
        Self {
            stack: __go_clone_0_0,
            conservative: __go_clone_1_0,
            buf: __go_clone_2_0,
            free_buf: __go_clone_3_0,
            cbuf: __go_clone_4_0,
            head: __go_clone_5_0,
            tail: __go_clone_6_0,
            nobjs: __go_clone_7_0,
            root: __go_clone_8_0,
        }
    }
}


impl Default for stackScanState {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(stack::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        let __go_default_5_0 = GoPtr::nil();
        let __go_default_6_0 = GoPtr::nil();
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = GoPtr::nil();
        Self {
            stack: __go_default_0_0,
            conservative: __go_default_1_0,
            buf: __go_default_2_0,
            free_buf: __go_default_3_0,
            cbuf: __go_default_4_0,
            head: __go_default_5_0,
            tail: __go_default_6_0,
            nobjs: __go_default_7_0,
            root: __go_default_8_0,
        }
    }
}

impl std::fmt::Display for stackScanState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.stack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.conservative.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", { let __guard = self.free_buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", { let __guard = self.cbuf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_5 = format!("{}", { if self.head.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_6 = format!("{}", { if self.tail.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_7 = format!("{}", (*self.nobjs.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", { if self.root.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8)
    }
}

impl GoJsonDecode for stackScanState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl stackObject {
    /// obj.r = r, but with no write barrier.
    ///
    ///go:nowritebarrier
    pub fn set_record(&self, r: GoPtr<crate::stack::stackObjectRecord>) {
                // Types of stack objects are always in read-only memory, not the heap.
                // So not using a write barrier is ok.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
    }
}

impl stackScanState {
    /// Add p as a potential pointer to a stack object.
    /// p must be a stack address.
    pub fn put_ptr(&mut self, p: Arc<Mutex<Option<usize>>>, conservative: Arc<Mutex<Option<bool>>>) {
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*self.stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*self.stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("address not a stack address".to_string()))));
    }
        let mut head = Arc::new(Mutex::new(Some(self.buf.clone())));
        if { let __v = (*conservative.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some(self.cbuf.clone()))).clone(); head = new_val; };
    }
        let mut buf_local: GoPtr<stackWorkBuf> = GoPtr::local({ let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v });
        if buf_local.is_nil() {
                // Initial setup.
        buf_local = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(getempty().addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = 0; *{ let __ptr_value = buf_local.with_mut(|__ptr_value| { let __field = __ptr_value.stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap() = Some(new_val); };
        *{ let __ptr_value = buf_local.with_mut(|__ptr_value| { let __field = __ptr_value.stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().next.clone(); __field }); __ptr_value }.lock().unwrap() = None;
        { let new_val = buf_local.clone().borrow(); let __dst = head.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = new_val; };
    } else if {
        let __tmp_x = ((*{ let __ptr_value = buf_local.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()) as i32);
        let __tmp_y = 252;
        __tmp_x == __tmp_y
    } {
        if { let __nil_target = self.free_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        buf_local = GoPtr::local(self.free_buf.clone());
        *self.free_buf.lock().unwrap() = None;
    } else {
        buf_local = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(getempty().addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
        { let new_val = 0; *{ let __ptr_value = buf_local.with_mut(|__ptr_value| { let __field = __ptr_value.stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = (*head.lock().unwrap().as_mut().unwrap()).clone(); buf_local.with_mut(|__ptr_value| { (*__ptr_value.stack_work_buf_hdr.lock().unwrap().as_mut().unwrap()).next = new_val; }); };
        { let new_val = buf_local.clone().borrow(); let __dst = head.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = new_val; };
    }
                // Initial setup.
        (*{ let __ptr_value = buf_local.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[((*{ let __ptr_value = buf_local.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize] = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let __target = { let __ptr_value = buf_local.with_mut(|__ptr_value| { let __field = __ptr_value.stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// Remove and return a potential pointer to a stack object.
    /// Returns 0 if there are no more pointers available.
    ///
    /// This prefers non-conservative pointers so we scan stack objects
    /// precisely if there are any non-conservative pointers to them.
    pub fn get_ptr(&mut self) -> (usize, bool) {
    let mut p: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut conservative: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        for head in &vec![Arc::new(Mutex::new(Some(self.buf.clone()))), Arc::new(Mutex::new(Some(self.cbuf.clone())))] {
        let mut buf_local = (*head.lock().unwrap().as_mut().unwrap()).clone();
        if { let __nil_result = (*buf_local.lock().unwrap()).is_none(); __nil_result } {
                // Never had any data.
        continue
    }
                // Never had any data.
        if { let __tmp_x = (*(*(*buf_local.lock().unwrap().as_ref().unwrap()).stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap()).nobj.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __nil_target = self.free_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Free old freeBuf.
        putempty(GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&self.free_buf.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
    }
                // Free old freeBuf.
                // Move buf to the freeBuf.
        { let new_val = buf_local.clone(); self.free_buf = new_val; };
        { let new_val = (*(*buf_local.lock().unwrap().as_mut().unwrap()).stack_work_buf_hdr.lock().unwrap().as_mut().unwrap()).next.clone(); buf_local = new_val; };
        { let new_val = buf_local.clone(); let __dst = head.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        if { let __nil_result = (*buf_local.lock().unwrap()).is_none(); __nil_result } {
                // No more data in this list.
        continue
    }
    }
                // Free old freeBuf.
                // Move buf to the freeBuf.
                // No more data in this list.
        { let __target = (*(*buf_local.lock().unwrap().as_mut().unwrap()).stack_work_buf_hdr.lock().unwrap().as_mut().unwrap().workbufhdr.lock().unwrap().as_mut().unwrap()).nobj.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        return ({ let __seq = { let __seq_holder = (*buf_local.lock().unwrap().as_ref().unwrap()).obj.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*(*(*buf_local.lock().unwrap().as_ref().unwrap()).stack_work_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap()).nobj.lock().unwrap().as_ref().unwrap())) as usize].clone() }, { let __left = head.clone(); let __right = Arc::new(Mutex::new(Some(self.cbuf.clone()))); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq });
    }
                // Never had any data.
                // Free old freeBuf.
                // Move buf to the freeBuf.
                // No more data in this list.
                // No more data in either list.
        if { let __nil_target = self.free_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        putempty(GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&self.free_buf.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
        *self.free_buf.lock().unwrap() = None;
    }
        (0, false)
    }

    /// addObject adds a stack object at addr of type typ to the set of stack objects.
    pub fn add_object(&mut self, addr: Arc<Mutex<Option<usize>>>, r: GoPtr<crate::stack::stackObjectRecord>) {
        let mut x: GoPtr<stackObjectBuf> = self.tail.clone();
        if x.is_nil() {
                // initial setup
        x = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(getempty().addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = GoPtr::nil(); x.with_mut(|__ptr_value| { (*__ptr_value.stack_object_buf_hdr.lock().unwrap().as_mut().unwrap()).next = new_val; }); };
        { let new_val = x.clone(); self.head = new_val; };
        { let new_val = x.clone(); self.tail = new_val; };
    }
                // initial setup
        if {
            let __go_cond_0 = { let __tmp_x = (*{ let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*self.stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = {
                        let __tmp_x = (*{ let __seq = { let __seq_holder = { let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*{ let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.off.lock().unwrap().as_ref().unwrap());
                        let __tmp_y = (*{ let __seq = { let __seq_holder = { let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*{ let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.size.lock().unwrap().as_ref().unwrap());
                        __tmp_x + __tmp_y
                    };
                    __tmp_x < __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
        throw(Arc::new(Mutex::new(Some("objects added out of order or overlapping".to_string()))));
    }
        if {
            let __tmp_x = ((*{ let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()) as i32);
            let __tmp_y = 63;
            __tmp_x == __tmp_y
        } {
                // full buffer - allocate a new buffer, add to end of linked list
        let mut y: GoPtr<stackObjectBuf> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(getempty().addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = GoPtr::nil(); y.with_mut(|__ptr_value| { (*__ptr_value.stack_object_buf_hdr.lock().unwrap().as_mut().unwrap()).next = new_val; }); };
        { let new_val = y.clone(); x.with_mut(|__ptr_value| { (*__ptr_value.stack_object_buf_hdr.lock().unwrap().as_mut().unwrap()).next = new_val; }); };
        { let new_val = y.clone(); self.tail = new_val; };
        x = y.clone();
    }
                // full buffer - allocate a new buffer, add to end of linked list
        let mut obj: Option<GoArrayElemPtr<stackObject, 63>> = Some(GoArrayElemPtr::new({ let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(), ((*{ let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize));
        { let __target = { let __ptr_value = x.with_mut(|__ptr_value| { let __field = __ptr_value.stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*self.stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*obj.as_ref().unwrap().borrow().as_ref().unwrap()).off.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = r.with_mut(|__ptr_value| __ptr_value.size.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*obj.as_ref().unwrap().borrow().as_ref().unwrap()).size.lock().unwrap() = __moved_val; };
        (*obj.as_ref().unwrap().borrow_mut().as_mut().unwrap()).set_record(r.clone());
                // obj.left and obj.right will be initialized by buildIndex before use.
        { let __target = self.nobjs.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// buildIndex initializes s.root to a binary search tree.
    /// It should be called after all addObject calls but before
    /// any call of findObject.
    pub fn build_index(&mut self) {
        { let (__tmp_0, __tmp_1, __tmp_2) = binary_search_tree(self.head.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __selector_holder = self.nobjs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); { let new_val = GoPtr::array_elem_opt(__tmp_0.clone()); self.root = new_val; } };
    }

    /// findObject returns the stack object containing address a, if any.
    /// Must have called buildIndex previously.
    pub fn find_object(&self, a: Arc<Mutex<Option<usize>>>) -> GoPtr<stackObject> {
        let mut off = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*self.stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as u32)));
        let mut obj: GoPtr<stackObject> = self.root.clone();
        loop {
        if obj.is_nil() {
        return GoPtr::nil();
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = obj.borrow(); __ptr_value.as_ref().unwrap().off.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        obj = { let __ptr_value = obj.borrow(); let __field_value = __ptr_value.as_ref().unwrap().left.clone(); __field_value };
        continue
    }
        if {
            let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = { let __tmp_x = (*{ let __ptr_value = obj.borrow(); __ptr_value.as_ref().unwrap().off.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = obj.borrow(); __ptr_value.as_ref().unwrap().size.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
            __tmp_x >= __tmp_y
        } {
        obj = { let __ptr_value = obj.borrow(); let __field_value = __ptr_value.as_ref().unwrap().right.clone(); __field_value };
        continue
    }
        return obj.clone();
    }
    }
}

impl stackObjectBuf {
}

impl stackObjectBufHdr {
}

impl stackWorkBuf {
}

impl stackWorkBufHdr {
}

fn __go_init_0() {
    if { let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<stackWorkBuf>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mgcwork::workbuf>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("stackWorkBuf too big".to_string()) as Box<dyn Any + Send + Sync>);
    }
    if { let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<stackObjectBuf>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mgcwork::workbuf>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("stackObjectBuf too big".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

/// Build a binary search tree with the n objects in the list
/// x.obj[idx], x.obj[idx+1], ..., x.next.obj[0], ...
/// Returns the root of that tree, and the buf+idx of the nth object after x.obj[idx].
/// (The first object that was not included in the binary search tree.)
/// If n == 0, returns nil, x.
pub fn binary_search_tree(mut x: GoPtr<stackObjectBuf>, mut idx: Arc<Mutex<Option<i32>>>, n: Arc<Mutex<Option<i32>>>) -> (Option<GoArrayElemPtr<stackObject, 63>>, GoPtr<stackObjectBuf>, i32) {
    let mut root: Option<GoArrayElemPtr<stackObject, 63>> = None;
    let mut restBuf: Arc<Mutex<Option<stackObjectBuf>>> = Arc::new(Mutex::new(None));
    let mut restIdx: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (None, x.clone(), { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    let mut left: GoPtr<stackObject> = GoPtr::nil();let mut right: GoPtr<stackObject> = GoPtr::nil();
    { let (__tmp_0, __tmp_1, __tmp_2) = binary_search_tree(
        x.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x / __tmp_y })))
    ); left = GoPtr::array_elem_opt(__tmp_0.clone()); x = __tmp_1.clone(); *idx.lock().unwrap() = Some(__tmp_2); };
    root = Some(GoArrayElemPtr::new({ let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(), ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
    { let mut guard = idx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if { let __tmp_x = ({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 63; __tmp_x == __tmp_y } {
        x = { let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().next.clone(); __field_value };
        { let new_val = 0; *idx.lock().unwrap() = Some(new_val); };
    }
    { let (__tmp_0, __tmp_1, __tmp_2) = binary_search_tree(
        x.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x / __tmp_y }; __tmp_x - __tmp_y }; let __tmp_y = 1; __tmp_x - __tmp_y })))
    ); right = GoPtr::array_elem_opt(__tmp_0.clone()); x = __tmp_1.clone(); *idx.lock().unwrap() = Some(__tmp_2); };
    { let new_val = left.clone(); (*root.as_ref().unwrap().borrow_mut().as_mut().unwrap()).left = new_val; };
    { let new_val = right.clone(); (*root.as_ref().unwrap().borrow_mut().as_mut().unwrap()).right = new_val; };
    return (root.clone(), x.clone(), { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}


impl GoValueClone for stackWorkBuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackWorkBufHdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackObjectBuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackObjectBufHdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackObject {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackScanState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
