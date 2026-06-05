use go2rust_stdlib_stubs::*;

use crate::{go_strconv_format_float, go_strconv_format_int};

use crate::codes::*;

use std::sync::{Arc, Mutex};

pub(crate) const __CODE_NAME_0: &'static str = "InvalidSyntaxTree";
pub(crate) const __CODE_NAME_1: &'static str = "TestBlankPkgNameMismatchedPkgNameInvalidPkgUseBadImportPathBrokenImportImportCRenamedUnusedImportInvalidInitCycleDuplicateDeclInvalidDeclCycleInvalidTypeCycleInvalidConstInitInvalidConstValInvalidConstTypeUntypedNilUseWrongAssignCountUnassignableOperandNoNewVarMultiValAssignOpInvalidIfaceAssignInvalidChanAssignIncompatibleAssignUnaddressableFieldAssignNotATypeInvalidArrayLenBlankIfaceMethodIncomparableMapKey";
pub(crate) const __CODE_NAME_2: &'static str = "InvalidPtrEmbedBadRecvInvalidRecvDuplicateFieldAndMethodDuplicateMethodInvalidBlankInvalidIotaMissingInitBodyInvalidInitSigInvalidInitDeclInvalidMainDeclTooManyValuesNotAnExprTruncatedFloatNumericOverflowUndefinedOpMismatchedTypesDivByZeroNonNumericIncDecUnaddressableOperandInvalidIndirectionNonIndexableOperandInvalidIndexSwappedSliceIndicesNonSliceableOperandInvalidSliceExprInvalidShiftCountInvalidShiftOperandInvalidReceiveInvalidSendDuplicateLitKeyMissingLitKeyInvalidLitIndexOversizeArrayLitMixedStructLitInvalidStructLitMissingLitFieldDuplicateLitFieldUnexportedLitFieldInvalidLitFieldUntypedLitInvalidLitAmbiguousSelectorUndeclaredImportedNameUnexportedNameUndeclaredNameMissingFieldOrMethodBadDotDotDotSyntaxNonVariadicDotDotDotMisplacedDotDotDot";
pub(crate) const __CODE_NAME_3: &'static str = "InvalidDotDotDotUncalledBuiltinInvalidAppendInvalidCapInvalidCloseInvalidCopyInvalidComplexInvalidDeleteInvalidImagInvalidLenSwappedMakeArgsInvalidMakeInvalidRealInvalidAssertImpossibleAssertInvalidConversionInvalidUntypedConversionBadOffsetofSyntaxInvalidOffsetofUnusedExprUnusedVarMissingReturnWrongResultCountOutOfScopeResultInvalidCondInvalidPostDecl";
pub(crate) const __CODE_NAME_4: &'static str = "InvalidIterVarInvalidRangeExprMisplacedBreakMisplacedContinueMisplacedFallthroughDuplicateCaseDuplicateDefaultBadTypeKeywordInvalidTypeSwitchInvalidExprSwitchInvalidSelectCaseUndeclaredLabelDuplicateLabelMisplacedLabelUnusedLabelJumpOverDeclJumpIntoBlockInvalidMethodExprWrongArgCountInvalidCallUnusedResultsInvalidDeferInvalidGoBadDeclRepeatedDeclInvalidUnsafeAddInvalidUnsafeSliceUnsupportedFeatureNotAGenericTypeWrongTypeArgCountCannotInferTypeArgsInvalidTypeArgInvalidInstanceCycleInvalidUnionMisplacedConstraintIfaceInvalidMethodTypeParamsMisplacedTypeParamInvalidUnsafeSliceDataInvalidUnsafeString";
pub(crate) const __CODE_NAME_5: &'static str = "InvalidClearTypeTooLargeInvalidMinMaxOperandTooNew";


pub(crate) static _Code_index_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u16; 29]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _Code_index_2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u16; 51]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _Code_index_3: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u16; 27]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _Code_index_4: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u16; 40]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _Code_index_5: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_Code_index_1.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_2.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_3.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_4.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_5.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_1.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 4 as u16, 16 as u16, 33 as u16, 46 as u16, 59 as u16, 71 as u16, 85 as u16, 97 as u16, 113 as u16, 126 as u16, 142 as u16, 158 as u16, 174 as u16, 189 as u16, 205 as u16, 218 as u16, 234 as u16, 253 as u16, 261 as u16, 277 as u16, 295 as u16, 312 as u16, 330 as u16, 354 as u16, 362 as u16, 377 as u16, 393 as u16, 411 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
    *_Code_index_2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 15 as u16, 22 as u16, 33 as u16, 56 as u16, 71 as u16, 83 as u16, 94 as u16, 109 as u16, 123 as u16, 138 as u16, 153 as u16, 166 as u16, 175 as u16, 189 as u16, 204 as u16, 215 as u16, 230 as u16, 239 as u16, 255 as u16, 275 as u16, 293 as u16, 312 as u16, 324 as u16, 343 as u16, 362 as u16, 378 as u16, 395 as u16, 414 as u16, 428 as u16, 439 as u16, 454 as u16, 467 as u16, 482 as u16, 498 as u16, 512 as u16, 528 as u16, 543 as u16, 560 as u16, 578 as u16, 593 as u16, 603 as u16, 613 as u16, 630 as u16, 652 as u16, 666 as u16, 680 as u16, 700 as u16, 718 as u16, 738 as u16, 756 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
    *_Code_index_3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 16 as u16, 31 as u16, 44 as u16, 54 as u16, 66 as u16, 77 as u16, 91 as u16, 104 as u16, 115 as u16, 125 as u16, 140 as u16, 151 as u16, 162 as u16, 175 as u16, 191 as u16, 208 as u16, 232 as u16, 249 as u16, 264 as u16, 274 as u16, 283 as u16, 296 as u16, 312 as u16, 328 as u16, 339 as u16, 354 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
    *_Code_index_4.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 14 as u16, 30 as u16, 44 as u16, 61 as u16, 81 as u16, 94 as u16, 110 as u16, 124 as u16, 141 as u16, 158 as u16, 175 as u16, 190 as u16, 204 as u16, 218 as u16, 229 as u16, 241 as u16, 254 as u16, 271 as u16, 284 as u16, 295 as u16, 308 as u16, 320 as u16, 329 as u16, 336 as u16, 348 as u16, 364 as u16, 382 as u16, 400 as u16, 415 as u16, 432 as u16, 451 as u16, 465 as u16, 485 as u16, 497 as u16, 521 as u16, 544 as u16, 562 as u16, 584 as u16, 603 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
    *_Code_index_5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 12 as u8, 24 as u8, 44 as u8, 50 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *_Code_index_1.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_2.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_3.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_4.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *_Code_index_5.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_0() {
    *_Code_index_1.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 4 as u16, 16 as u16, 33 as u16, 46 as u16, 59 as u16, 71 as u16, 85 as u16, 97 as u16, 113 as u16, 126 as u16, 142 as u16, 158 as u16, 174 as u16, 189 as u16, 205 as u16, 218 as u16, 234 as u16, 253 as u16, 261 as u16, 277 as u16, 295 as u16, 312 as u16, 330 as u16, 354 as u16, 362 as u16, 377 as u16, 393 as u16, 411 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_1() {
    *_Code_index_2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 15 as u16, 22 as u16, 33 as u16, 56 as u16, 71 as u16, 83 as u16, 94 as u16, 109 as u16, 123 as u16, 138 as u16, 153 as u16, 166 as u16, 175 as u16, 189 as u16, 204 as u16, 215 as u16, 230 as u16, 239 as u16, 255 as u16, 275 as u16, 293 as u16, 312 as u16, 324 as u16, 343 as u16, 362 as u16, 378 as u16, 395 as u16, 414 as u16, 428 as u16, 439 as u16, 454 as u16, 467 as u16, 482 as u16, 498 as u16, 512 as u16, 528 as u16, 543 as u16, 560 as u16, 578 as u16, 593 as u16, 603 as u16, 613 as u16, 630 as u16, 652 as u16, 666 as u16, 680 as u16, 700 as u16, 718 as u16, 738 as u16, 756 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_2() {
    *_Code_index_3.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 16 as u16, 31 as u16, 44 as u16, 54 as u16, 66 as u16, 77 as u16, 91 as u16, 104 as u16, 115 as u16, 125 as u16, 140 as u16, 151 as u16, 162 as u16, 175 as u16, 191 as u16, 208 as u16, 232 as u16, 249 as u16, 264 as u16, 274 as u16, 283 as u16, 296 as u16, 312 as u16, 328 as u16, 339 as u16, 354 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_3() {
    *_Code_index_4.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 14 as u16, 30 as u16, 44 as u16, 61 as u16, 81 as u16, 94 as u16, 110 as u16, 124 as u16, 141 as u16, 158 as u16, 175 as u16, 190 as u16, 204 as u16, 218 as u16, 229 as u16, 241 as u16, 254 as u16, 271 as u16, 284 as u16, 295 as u16, 308 as u16, 320 as u16, 329 as u16, 336 as u16, 348 as u16, 364 as u16, 382 as u16, 400 as u16, 415 as u16, 432 as u16, 451 as u16, 465 as u16, 485 as u16, 497 as u16, 521 as u16, 544 as u16, 562 as u16, 584 as u16, 603 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_4() {
    *_Code_index_5.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 12 as u8, 24 as u8, 44 as u8, 50 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::codes::Code {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut __self = self.clone();
        if { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Code(Arc::new(Mutex::new(Some(-1 as i32)))); __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(__CODE_NAME_0.to_string())));
        } else if { let __tmp_x = Code(Arc::new(Mutex::new(Some(1 as i32)))); let __tmp_y = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Code(Arc::new(Mutex::new(Some(28 as i32)))); __tmp_x <= __tmp_y } {
            { let __rhs = 1; let mut guard = __self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - __rhs); };
            return Arc::new(Mutex::new(Some({ let __s = &(__CODE_NAME_1); let __low = ({ let __seq = { let __seq_holder = _Code_index_1.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*__self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Code_index_1.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*__self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })));
        } else if { let __tmp_x = Code(Arc::new(Mutex::new(Some(30 as i32)))); let __tmp_y = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Code(Arc::new(Mutex::new(Some(79 as i32)))); __tmp_x <= __tmp_y } {
            { let __rhs = 30; let mut guard = __self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - __rhs); };
            return Arc::new(Mutex::new(Some({ let __s = &(__CODE_NAME_2); let __low = ({ let __seq = { let __seq_holder = _Code_index_2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*__self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Code_index_2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*__self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })));
        } else if { let __tmp_x = Code(Arc::new(Mutex::new(Some(81 as i32)))); let __tmp_y = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Code(Arc::new(Mutex::new(Some(106 as i32)))); __tmp_x <= __tmp_y } {
            { let __rhs = 81; let mut guard = __self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - __rhs); };
            return Arc::new(Mutex::new(Some({ let __s = &(__CODE_NAME_3); let __low = ({ let __seq = { let __seq_holder = _Code_index_3.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*__self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Code_index_3.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*__self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })));
        } else if { let __tmp_x = Code(Arc::new(Mutex::new(Some(108 as i32)))); let __tmp_y = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Code(Arc::new(Mutex::new(Some(146 as i32)))); __tmp_x <= __tmp_y } {
            { let __rhs = 108; let mut guard = __self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - __rhs); };
            return Arc::new(Mutex::new(Some({ let __s = &(__CODE_NAME_4); let __low = ({ let __seq = { let __seq_holder = _Code_index_4.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*__self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Code_index_4.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*__self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })));
        } else if { let __tmp_x = Code(Arc::new(Mutex::new(Some(148 as i32)))); let __tmp_y = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*__self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Code(Arc::new(Mutex::new(Some(151 as i32)))); __tmp_x <= __tmp_y } {
            { let __rhs = 148; let mut guard = __self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() - __rhs); };
            return Arc::new(Mutex::new(Some({ let __s = &(__CODE_NAME_5); let __low = ({ let __seq = { let __seq_holder = _Code_index_5.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*__self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }) as usize; let __high = ({ let __seq = { let __seq_holder = _Code_index_5.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*__self.0.lock().unwrap().as_ref().unwrap()) + 1) as usize].clone() }) as usize; __s[__low..__high].to_string() })));
        } else {
            return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "Code(".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(go_strconv_format_int((*Arc::new(Mutex::new(Some((*__self.0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()) as i64, 10 as i32)))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
