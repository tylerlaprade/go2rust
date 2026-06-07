use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
use crate::floatconv::*;
use crate::floatmarsh::*;
use crate::ftoa::*;
use crate::int::*;
use crate::intconv::*;
use crate::intmarsh::*;
use crate::nat::*;
use crate::natconv::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const RAT_GOB_VERSION: u8 = 1;


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: GoMutex,
    pub table: Arc<Mutex<Option<[divisor; 64]>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), table: { let __guard = self.table.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: GoMutex::new(), table: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.table))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type cacheBase10 = AnonymousStruct1;


impl crate::rat::Rat {
    /// GobEncode implements the [encoding/gob.GobEncoder] interface.
    pub fn gob_encode(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        let mut buf = Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = 5; let __tmp_y = ({ let __tmp_x = (({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x + __tmp_y }) as i32); let __tmp_y = 8; __tmp_x * __tmp_y } as i32); __tmp_x + __tmp_y }) as usize])));
        let mut i = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).bytes(buf.clone());
        let mut j = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(i) as usize].to_vec() }))));
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = i; let __tmp_y = j; __tmp_x - __tmp_y })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u32 as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // this should never happen
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Rat.GobEncode: numerator too large".to_string())))));
    }
                // this should never happen
        byteorder::b_e_put_uint32(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = ({ let __tmp_x = j; let __tmp_y = 4; __tmp_x - __tmp_y }) as usize; __seq[__low..(j) as usize].to_vec() }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u32))));
        { let __rhs = 5; j = j - __rhs; };
        let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = (RAT_GOB_VERSION as u8); let __tmp_y = 1; __tmp_x << __tmp_y })));
        if (*(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap().as_ref().unwrap()) {
        { let __rhs = 1 as u8; let mut guard = b.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        (*buf.lock().unwrap().as_mut().unwrap())[(j) as usize] = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
        return (Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(j) as usize..].to_vec() }))), Arc::new(Mutex::new(None)));
    }

    /// GobDecode implements the [encoding/gob.GobDecoder] interface.
    pub fn gob_decode(&mut self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Other side sent a nil or default value.
        { let new_val = Rat { a: Arc::new(Mutex::new(Some(Default::default()))), b: Arc::new(Mutex::new(Some(Default::default()))) }; *self = new_val; };
        return Arc::new(Mutex::new(None));
    }
                // Other side sent a nil or default value.
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 5; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Rat.GobDecode: buffer too small".to_string()))));
    }
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; let __tmp_y = RAT_GOB_VERSION as u8; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("Rat.GobDecode: encoding version {} not supported", { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y })))));
    }
        const j: i32 = 1 + 4;

        let mut ln = byteorder::b_e_uint32(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = ({ let __tmp_x = j; let __tmp_y = 4; __tmp_x - __tmp_y }) as usize; __seq[__low..(j) as usize].to_vec() }))));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(ln as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((math::MAX_INT as u64) - (j as u64)) as u64; __tmp_x > __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Rat.GobDecode: invalid length".to_string()))));
    }
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = 5; let __tmp_y = (*Arc::new(Mutex::new(Some(ln as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Rat.GobDecode: buffer too small".to_string()))));
    }
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *(*self.a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(j) as usize..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        { let new_val = (*(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        return Arc::new(Mutex::new(None));
    }

    /// AppendText implements the [encoding.TextAppender] interface.
    pub fn append_text(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if self.is_int() {
        return (*self.a.lock().unwrap().as_ref().unwrap()).append_text(b.clone());
    }
        (self.marshal(b.clone()), Arc::new(Mutex::new(None)))
    }

    /// MarshalText implements the [encoding.TextMarshaler] interface.
    pub fn marshal_text(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut text: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        self.append_text(Arc::new(Mutex::new(None)))
    }

    /// UnmarshalText implements the [encoding.TextUnmarshaler] interface.
    pub fn unmarshal_text(&mut self, text: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
                // TODO(gri): get rid of the []byte/string conversion
        {
        let (_, mut ok) = self.set_string(Arc::new(Mutex::new(Some(String::from_utf8((*text.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))));;
        if !ok {
            return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("math/big: cannot unmarshal {:?} into a *big.Rat", format_slice(&text))))));;
        }
    }
        return Arc::new(Mutex::new(None));
    }
}