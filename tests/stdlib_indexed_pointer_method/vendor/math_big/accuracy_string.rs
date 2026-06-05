use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone, go_strconv_format_float, go_strconv_format_int};

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
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::sync::{Arc, Mutex};

pub(crate) const __ACCURACY_NAME: &'static str = "BelowExactAbove";


pub(crate) static _Accuracy_index: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 4]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_Accuracy_index.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Accuracy_index.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 5 as u8, 10 as u8, 15 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_Accuracy_index.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_0() {
    *_Accuracy_index.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 5 as u8, 10 as u8, 15 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::float::Accuracy {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut __self = self.clone();
        { let __rhs = -1 as i8; let mut guard = __self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - __rhs); };
        if { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::float::Accuracy(Arc::new(Mutex::new(Some(0 as i8)))); __tmp_x < __tmp_y } || { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::float::Accuracy(Arc::new(Mutex::new(Some({ let __tmp_x = 4; let __tmp_y = 1; __tmp_x - __tmp_y } as i8)))); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "Accuracy(".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(go_strconv_format_int((*Arc::new(Mutex::new(Some((((*__self.0.lock().unwrap().as_ref().unwrap()) + -1)) as i64))).lock().unwrap().as_ref().unwrap()) as i64, 10 as i32)))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
    }
        Arc::new(Mutex::new(Some({ let __s = &(__ACCURACY_NAME); let __low = ({ let __seq = { let __seq_holder = _Accuracy_index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*__self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Accuracy_index.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*__self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })))
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
