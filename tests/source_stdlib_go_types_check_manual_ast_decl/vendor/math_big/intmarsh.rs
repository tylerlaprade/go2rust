use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_strconv_format_float, go_strconv_format_int};

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
use crate::nat::*;
use crate::natconv::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

pub(crate) const INT_GOB_VERSION: u8 = 1;


impl crate::int::Int {
    /// GobEncode implements the [encoding/gob.GobEncoder] interface.
    pub fn gob_encode(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        let mut buf = Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = 1; let __tmp_y = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 8; __tmp_x * __tmp_y } as i32); __tmp_x + __tmp_y }) as usize])));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.abs.lock().unwrap().as_ref().unwrap()).bytes(buf.clone()); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = (INT_GOB_VERSION as u8); let __tmp_y = 1; __tmp_x << __tmp_y })));
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let __rhs = 1 as u8; let mut guard = b.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
        return (Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))), Arc::new(Mutex::new(None)));
    }

    /// GobDecode implements the [encoding/gob.GobDecoder] interface.
    pub fn gob_decode(&mut self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Other side sent a nil or default value.
        { let new_val = Int { neg: Arc::new(Mutex::new(Some(false))), abs: Arc::new(Mutex::new(Some(Default::default()))) }; *self = new_val; };
        return Arc::new(Mutex::new(None));
    }
                // Other side sent a nil or default value.
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; let __tmp_y = INT_GOB_VERSION as u8; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("Int.GobDecode: encoding version {} not supported", { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y })))));
    }
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.abs.lock().unwrap().as_ref().unwrap()).set_bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.abs.lock().unwrap() = __moved_val; };
        return Arc::new(Mutex::new(None));
    }

    /// AppendText implements the [encoding.TextAppender] interface.
    pub fn append_text(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut text: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        (self.append(b.clone(), Arc::new(Mutex::new(Some(10)))), Arc::new(Mutex::new(None)))
    }

    /// MarshalText implements the [encoding.TextMarshaler] interface.
    pub fn marshal_text(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut text: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        self.append_text(Arc::new(Mutex::new(None)))
    }

    /// UnmarshalText implements the [encoding.TextUnmarshaler] interface.
    pub fn unmarshal_text(&mut self, text: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let (_, mut ok) = self.set_from_scanner({ let __arg = bytes::new_reader(text.clone()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<io_ByteScanner> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, Arc::new(Mutex::new(Some(0))));;
        if !ok {
            return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("math/big: cannot unmarshal {:?} into a *big.Int", format_slice(&text))))));;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// MarshalJSON implements the [encoding/json.Marshaler] interface.
    pub fn marshal_j_s_o_n(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(Some(("null".to_string()).as_bytes().to_vec()))), Arc::new(Mutex::new(None)));
    }
        ((*self.abs.lock().unwrap().as_ref().unwrap()).itoa(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(10)))), Arc::new(Mutex::new(None)))
    }

    /// UnmarshalJSON implements the [encoding/json.Unmarshaler] interface.
    pub fn unmarshal_j_s_o_n(&mut self, text: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
                // Ignore null, like in the main JSON package.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*text.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "null".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        self.unmarshal_text(text.clone())
    }
}