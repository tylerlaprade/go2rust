use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::digit::*;
use crate::graphic::*;
use crate::letter::*;
use crate::tables::*;

use std::sync::{Arc, Mutex};

pub static TurkishCase: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::letter::SpecialCase>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _TurkishCase: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::letter::SpecialCase>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static AzeriCase: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::letter::SpecialCase>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *TurkishCase.lock().unwrap() = Some(Default::default());
    *_TurkishCase.lock().unwrap() = Some(Default::default());
    *AzeriCase.lock().unwrap() = Some(Default::default());
    *_TurkishCase.lock().unwrap() = Some(SpecialCase(Arc::new(Mutex::new(Some(vec![CaseRange { lo: Arc::new(Mutex::new(Some(0x0049 as u32))), hi: Arc::new(Mutex::new(Some(0x0049 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }, CaseRange { lo: Arc::new(Mutex::new(Some(0x0069 as u32))), hi: Arc::new(Mutex::new(Some(0x0069 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }, CaseRange { lo: Arc::new(Mutex::new(Some(0x0130 as u32))), hi: Arc::new(Mutex::new(Some(0x0130 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }, CaseRange { lo: Arc::new(Mutex::new(Some(0x0131 as u32))), hi: Arc::new(Mutex::new(Some(0x0131 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }])))));
    *TurkishCase.lock().unwrap() = Some((*_TurkishCase.lock().unwrap().as_ref().unwrap()).clone());
    *AzeriCase.lock().unwrap() = Some((*_TurkishCase.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *TurkishCase.lock().unwrap() = Some(Default::default());
    *_TurkishCase.lock().unwrap() = Some(Default::default());
    *AzeriCase.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_0() {
    *_TurkishCase.lock().unwrap() = Some(SpecialCase(Arc::new(Mutex::new(Some(vec![CaseRange { lo: Arc::new(Mutex::new(Some(0x0049 as u32))), hi: Arc::new(Mutex::new(Some(0x0049 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }, CaseRange { lo: Arc::new(Mutex::new(Some(0x0069 as u32))), hi: Arc::new(Mutex::new(Some(0x0069 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }, CaseRange { lo: Arc::new(Mutex::new(Some(0x0130 as u32))), hi: Arc::new(Mutex::new(Some(0x0130 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }, CaseRange { lo: Arc::new(Mutex::new(Some(0x0131 as u32))), hi: Arc::new(Mutex::new(Some(0x0131 as u32))), delta: Arc::new(Mutex::new(Some(d { /* ERROR: Type information required for positional struct literal */ ..Default::default() }))), ..Default::default() }])))));
}


pub(crate) fn __go_init_order_1() {
    *TurkishCase.lock().unwrap() = Some((*_TurkishCase.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_2() {
    *AzeriCase.lock().unwrap() = Some((*_TurkishCase.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
