use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::float::*;
use crate::floatconv::*;
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
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

pub(crate) const FLOAT_GOB_VERSION: u8 = 1;


impl crate::float::Float {
    /// GobEncode implements the [encoding/gob.GobEncoder] interface.
    /// The [Float] value and all its attributes (precision,
    /// rounding mode, accuracy) are marshaled.
    pub fn gob_encode(&self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
                // determine max. space (bytes) required for encoding
        let mut sz = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = 4; __tmp_x + __tmp_y })));
        let mut n = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::float::form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // add space for mantissa and exponent
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((__W as u32) - (1 as u32)) as u32; __tmp_x + __tmp_y }); let __tmp_y = __W as u32; __tmp_x / __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
                // actual mantissa slice could be shorter (trailing 0's) or longer (unused bits):
                // - if shorter, only encode the words present
                // - if longer, cut off unused words when encoding in bytes
                //   (in practice, this should never happen since rounding
                //   takes care of it, but be safe and do it always)
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32; *n.lock().unwrap() = Some(new_val); };
    }
                // len(x.mant) >= n
        { let __rhs = { let __tmp_x = 4; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let mut guard = sz.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // add space for mantissa and exponent
                // required mantissa length in words for given precision
                // actual mantissa slice could be shorter (trailing 0's) or longer (unused bits):
                // - if shorter, only encode the words present
                // - if longer, cut off unused words when encoding in bytes
                //   (in practice, this should never happen since rounding
                //   takes care of it, but be safe and do it always)
                // len(x.mant) >= n
                // exp + mant
        let mut buf = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
        (*buf.lock().unwrap().as_mut().unwrap())[(0) as usize] = FLOAT_GOB_VERSION as u8;
        let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & 7)) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 5; __tmp_x << __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((((((*(*self.acc.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1)) & 3)) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3; __tmp_x << __tmp_y }; __tmp_x | __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((((*(*self.form.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & 3)) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }; __tmp_x | __tmp_y })));
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let __rhs = 1 as u8; let mut guard = b.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        (*buf.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v };
        byteorder::b_e_put_uint32(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize..].to_vec() }))), { let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::float::form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
        byteorder::b_e_put_uint32(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(6) as usize..].to_vec() }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))));
        crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); let __low = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }) as usize; __seq[__low..].to_vec() })))).bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(10) as usize..].to_vec() }))));
    }
                // cut off unused trailing words
        return (buf.clone(), Arc::new(Mutex::new(None)));
    }

    /// GobDecode implements the [encoding/gob.GobDecoder] interface.
    /// The result is rounded per the precision and rounding mode of
    /// z unless z's precision is 0, in which case z is set exactly
    /// to the decoded value.
    pub fn gob_decode(&mut self, buf: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Other side sent a nil or default value.
        { let new_val = Float { prec: Arc::new(Mutex::new(Some(0))), mode: Arc::new(Mutex::new(Some(crate::float::RoundingMode(Arc::new(Mutex::new(Some(0))))))), acc: Arc::new(Mutex::new(Some(crate::float::Accuracy(Arc::new(Mutex::new(Some(0))))))), form: Arc::new(Mutex::new(Some(crate::float::form(Arc::new(Mutex::new(Some(0))))))), neg: Arc::new(Mutex::new(Some(false))), mant: Arc::new(Mutex::new(Some(Default::default()))), exp: Arc::new(Mutex::new(Some(0))) }; *self = new_val; };
        return Arc::new(Mutex::new(None));
    }
                // Other side sent a nil or default value.
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 6; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Float.GobDecode: buffer too small".to_string()))));
    }
        if { let __tmp_x = { let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = FLOAT_GOB_VERSION as u8; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("Float.GobDecode: encoding version {} not supported", { let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })))));
    }
        let mut oldPrec = Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut oldMode = Arc::new(Mutex::new(Some({ let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })));
        { let new_val = crate::float::RoundingMode(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x >> __tmp_y }); let __tmp_y = 7 as u8; __tmp_x & __tmp_y } as u8)))); *self.mode.lock().unwrap() = Some(new_val); };
        { let new_val = crate::float::Accuracy(Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x >> __tmp_y }); let __tmp_y = 3 as u8; __tmp_x & __tmp_y } as i8 - 1))))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = crate::float::form(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }); let __tmp_y = 3 as u8; __tmp_x & __tmp_y } as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        { let new_val = byteorder::b_e_uint32(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize..].to_vec() })))); *self.prec.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::float::form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 10; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Float.GobDecode: buffer too small for finite form float".to_string()))));
    }
        { let new_val = Arc::new(Mutex::new(Some(byteorder::b_e_uint32(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(6) as usize..].to_vec() })))) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.exp.lock().unwrap() = __moved_val; };
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).set_bytes(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(10) as usize..].to_vec() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = { let __v = (*oldPrec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        { let new_val = oldMode.lock().unwrap().as_ref().unwrap().clone(); *self.mode.lock().unwrap() = Some(new_val); };
        self.set_prec(Arc::new(Mutex::new(Some((*oldPrec.lock().unwrap().as_ref().unwrap()) as u64))));
    }
        {
        let mut msg = self.validate0();;
        if { let __tmp_x = (*msg.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(format!("{}{}", "Float.GobDecode: ".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v })))));;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// AppendText implements the [encoding.TextAppender] interface.
    /// Only the [Float] value is marshaled (in full precision), other
    /// attributes such as precision or accuracy are ignored.
    pub fn append_text(&mut self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return ({ let __append_target = b.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("<nil>".to_string().as_bytes().iter().cloned()); __append_target.clone() }, Arc::new(Mutex::new(None)));
    }
        (self.append(b.clone(), Arc::new(Mutex::new(Some(('g' as i32) as u8))), Arc::new(Mutex::new(Some(-1)))), Arc::new(Mutex::new(None)))
    }

    /// MarshalText implements the [encoding.TextMarshaler] interface.
    /// Only the [Float] value is marshaled (in full precision), other
    /// attributes such as precision or accuracy are ignored.
    pub fn marshal_text(&mut self) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut text: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        self.append_text(Arc::new(Mutex::new(None)))
    }

    /// UnmarshalText implements the [encoding.TextUnmarshaler] interface.
    /// The result is rounded per the precision and rounding mode of z.
    /// If z's precision is 0, it is changed to 64 before rounding takes
    /// effect.
    pub fn unmarshal_text(&mut self, text: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
                // TODO(gri): get rid of the []byte/string conversion
        let (_, _, mut err) = self.parse(Arc::new(Mutex::new(Some(String::from_utf8((*text.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), Arc::new(Mutex::new(Some(0))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("math/big: cannot unmarshal {:?} into a *big.Float ({})", format_slice(&text), format!("{}", (*err.lock().unwrap().as_ref().unwrap()))))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return err.clone();
    }
}