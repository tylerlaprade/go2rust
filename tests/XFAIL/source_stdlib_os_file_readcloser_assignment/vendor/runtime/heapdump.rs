use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{malloc::{__PAGE_SIZE}, r#type::{_type}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const FIELD_KIND_EOL: i32 = 0;
pub(crate) const FIELD_KIND_PTR: i32 = 1;
pub(crate) const FIELD_KIND_IFACE: i32 = 2;
pub(crate) const FIELD_KIND_EFACE: i32 = 3;
pub(crate) const TAG_E_O_F: i32 = 0;
pub(crate) const TAG_OBJECT: i32 = 1;
pub(crate) const TAG_OTHER_ROOT: i32 = 2;
pub(crate) const TAG_TYPE: i32 = 3;
pub(crate) const TAG_GOROUTINE: i32 = 4;
pub(crate) const TAG_STACK_FRAME: i32 = 5;
pub(crate) const TAG_PARAMS: i32 = 6;
pub(crate) const TAG_FINALIZER: i32 = 7;
pub(crate) const TAG_ITAB: i32 = 8;
pub(crate) const TAG_O_S_THREAD: i32 = 9;
pub(crate) const TAG_MEM_STATS: i32 = 10;
pub(crate) const TAG_QUEUED_FINALIZER: i32 = 11;
pub(crate) const TAG_DATA: i32 = 12;
pub(crate) const TAG_B_S_S: i32 = 13;
pub(crate) const TAG_DEFER: i32 = 14;
pub(crate) const TAG_PANIC: i32 = 15;
pub(crate) const TAG_MEM_PROF: i32 = 16;
pub(crate) const TAG_ALLOC_SAMPLE: i32 = 17;


pub(crate) const BUF_SIZE: i32 = 4096;


pub(crate) const TYPE_CACHE_BUCKETS: i32 = 256;
pub(crate) const TYPE_CACHE_ASSOC: i32 = 4;


#[derive(Clone)]
pub struct typeCacheBucket {
    pub t: Arc<Mutex<Option<[GoPtr<internal_abi::r#type::Type>; 4]>>>,
}

impl typeCacheBucket {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            t: __go_clone_0_0,
        }
    }
}


impl Default for typeCacheBucket {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil()))));
        Self {
            t: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for typeCacheBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.t.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for typeCacheBucket {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static dumpfd: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static tmpbuf: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static buf: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 4096]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static nbuf: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static typecache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[typeCacheBucket; 256]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static freemark: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[bool; 1024]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static dumphdr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *dumpfd.lock().unwrap() = Some(0);
    *tmpbuf.lock().unwrap() = Some(vec![]);
    *buf.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *nbuf.lock().unwrap() = Some(0);
    *typecache.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *freemark.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *dumphdr.lock().unwrap() = Some(vec![]);
    *dumphdr.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("go1.7 heap dump\n".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *dumpfd.lock().unwrap() = Some(0);
    *tmpbuf.lock().unwrap() = Some(vec![]);
    *buf.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *nbuf.lock().unwrap() = Some(0);
    *typecache.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *freemark.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *dumphdr.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_6() {
    *dumphdr.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("go1.7 heap dump\n".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for typeCacheBucket {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
