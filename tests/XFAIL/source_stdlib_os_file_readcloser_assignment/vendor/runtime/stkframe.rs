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
    panic::{throw},
    print::{hex},
    r#extern::{G_O_A_R_C_H},
    stack::{STACK_DEBUG, bitvector, stackObjectRecord},
    stubs::{add, noescape},
    symtab::{funcInfo, funcdata, funcname, pcdatavalue, stackmap, stackmapdata},
    traceback::{USES_L_R},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A stkframe holds information about a single physical stack frame.
#[derive(Clone)]
pub struct stkframe {
    pub r#fn: Arc<Mutex<Option<funcInfo>>>,
    pub pc: Arc<Mutex<Option<usize>>>,
    pub continpc: Arc<Mutex<Option<usize>>>,
    pub lr: Arc<Mutex<Option<usize>>>,
    pub sp: Arc<Mutex<Option<usize>>>,
    pub fp: Arc<Mutex<Option<usize>>>,
    pub varp: Arc<Mutex<Option<usize>>>,
    pub argp: Arc<Mutex<Option<usize>>>,
}

impl stkframe {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.continpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.fp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.varp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.argp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#fn: __go_clone_0_0,
            pc: __go_clone_1_0,
            continpc: __go_clone_2_0,
            lr: __go_clone_3_0,
            sp: __go_clone_4_0,
            fp: __go_clone_5_0,
            varp: __go_clone_6_0,
            argp: __go_clone_7_0,
        }
    }
}


impl Default for stkframe {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(funcInfo::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#fn: __go_default_0_0,
            pc: __go_default_1_0,
            continpc: __go_default_2_0,
            lr: __go_default_3_0,
            sp: __go_default_4_0,
            fp: __go_default_5_0,
            varp: __go_default_6_0,
            argp: __go_default_7_0,
        }
    }
}

impl std::fmt::Display for stkframe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#fn.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.continpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.lr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.sp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.fp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.varp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.argp.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7)
    }
}

impl GoJsonDecode for stkframe {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// reflectMethodValue is a partial duplicate of reflect.makeFuncImpl
/// and reflect.methodValue.
#[derive(Debug, Clone)]
pub struct reflectMethodValue {
    pub r#fn: Arc<Mutex<Option<usize>>>,
    pub stack: Arc<Mutex<Option<bitvector>>>,
    pub arg_len: Arc<Mutex<Option<usize>>>,
}

impl reflectMethodValue {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.stack.clone();
        let __go_clone_2_0 = { let __guard = self.arg_len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#fn: __go_clone_0_0,
            stack: __go_clone_1_0,
            arg_len: __go_clone_2_0,
        }
    }
}


impl Default for reflectMethodValue {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#fn: __go_default_0_0,
            stack: __go_default_1_0,
            arg_len: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for reflectMethodValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#fn.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.stack.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.arg_len.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for reflectMethodValue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static methodValueCallFrameObjs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[crate::stack::stackObjectRecord; 1]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *methodValueCallFrameObjs.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
}


pub(crate) fn __go_zero_globals() {
    *methodValueCallFrameObjs.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
}


impl stkframe {
    /// argBytes returns the argument frame size for a call to frame.fn.
    pub fn arg_bytes(&self) -> usize {
        if {
            let __tmp_x = (*(*self.r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().args.lock().unwrap().as_ref().unwrap());
            let __tmp_y = internal_abi::ARGS_SIZE_UNKNOWN as i32;
            __tmp_x != __tmp_y
        } {
        return (*Arc::new(Mutex::new(Some({ let __selector_holder = (*self.r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().args.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
    }
                // This is an uncommon and complicated case. Fall back to fully
                // fetching the argument map to compute its size.
        let (mut argMap, _) = self.arg_map_internal();
        return { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*argMap.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y };
    }

    /// argMapInternal is used internally by stkframe to fetch special
    /// argument maps.
    ///
    /// argMap.n is always populated with the size of the argument map.
    ///
    /// argMap.bytedata is only populated for dynamic argument maps (used
    /// by reflect). If the caller requires the argument map, it should use
    /// this if non-nil, and otherwise fetch the argument map using the
    /// current PC.
    ///
    /// hasReflectStackObj indicates that this frame also has a reflect
    /// function stack object, which the caller must synthesize.
    pub fn arg_map_internal(&self) -> (Arc<Mutex<Option<crate::stack::bitvector>>>, bool) {
    let mut argMap: Arc<Mutex<Option<bitvector>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut hasReflectStackObj: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = self.r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).args.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_abi::ARGS_SIZE_UNKNOWN as i32; __tmp_x != __tmp_y } {
        { let new_val = { let __tmp_x = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).args.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as i32; __tmp_x / __tmp_y }; *(*argMap.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
        return (argMap.clone(), (*hasReflectStackObj.lock().unwrap().as_ref().unwrap()));
    }
                // Extract argument bitmaps for reflect stubs from the calls they made to reflect.
        { let _switch_val = { let __v = funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == ("reflect.makeFuncStub".to_string()) || _switch_val == ("reflect.methodValueCall".to_string()) {
                        // These take a *reflect.methodValue as their
                        // context register and immediately save it to 0(SP).
                        // Get the methodValue from 0(SP).
            let mut arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.sp.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_runtime_sys::MIN_FRAME_SIZE as usize; __tmp_x + __tmp_y })));
            let mut minSP = Arc::new(Mutex::new(Some({ let __selector_holder = self.fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            if !USES_L_R {
                // The CALL itself pushes a word.
                // Undo that adjustment.
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = minSP.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                        // The CALL itself pushes a word.
                        // Undo that adjustment.
            if { let __tmp_x = { let __v = (*arg0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*minSP.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // The function hasn't started yet.
                // This only happens if f was the
                // start function of a new goroutine
                // that hasn't run yet *and* f takes
                // no arguments and has no results
                // (otherwise it will get wrapped in a
                // closure). In this case, we can't
                // reach into its locals because it
                // doesn't have locals yet, but we
                // also know its argument map is
                // empty.
        if { let __tmp_x = (*self.pc.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: confused by ".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", ": no frame (sp=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = self.sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " fp=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = self.fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", ") at entry+".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.pc.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x - __tmp_y } as u64)))));
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        throw(Arc::new(Mutex::new(Some("reflect mismatch".to_string()))));
    }
        return (
            Arc::new(Mutex::new(Some(crate::stack::bitvector { ..Default::default() }))),
            false
        );
    }
                        // The function hasn't started yet.
                        // This only happens if f was the
                        // start function of a new goroutine
                        // that hasn't run yet *and* f takes
                        // no arguments and has no results
                        // (otherwise it will get wrapped in a
                        // closure). In this case, we can't
                        // reach into its locals because it
                        // doesn't have locals yet, but we
                        // also know its argument map is
                        // empty.
                        // No locals, so also no stack objects
            { let new_val = true; *hasReflectStackObj.lock().unwrap() = Some(new_val); };
            let mut mv = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*arg0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Arc<Mutex<Option<reflectMethodValue>>>>(unimplemented!("unsafe.Pointer conversion to Arc<Mutex<Option<reflectMethodValue>>>")) } })).lock().unwrap().as_mut().unwrap()).clone();
                        // Figure out whether the return values are valid.
                        // Reflect will update this value after it copies
                        // in the return values.
            let mut retValid = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*arg0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((4 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<bool>(unimplemented!("unsafe.Pointer conversion to bool")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
            if { let __tmp_x = (*{ let __field = (*mv.lock().unwrap().as_ref().unwrap()).r#fn.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: confused by ".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        throw(Arc::new(Mutex::new(Some("reflect mismatch".to_string()))));
    }
            { let new_val = { let __v = (*(*mv.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).clone(); __v }; *argMap.lock().unwrap() = Some(new_val); };
            if !{ let __v = (*retValid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // argMap.n includes the results, but
                // those aren't valid, so drop them.
        let mut n = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*{ let __field = (*mv.lock().unwrap().as_ref().unwrap()).arg_len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) - (1 as usize)) as usize; __tmp_x & ! __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }) as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*argMap.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *(*argMap.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    }
    }
        }
    }
                // These take a *reflect.methodValue as their
                // context register and immediately save it to 0(SP).
                // Get the methodValue from 0(SP).
                // The CALL itself pushes a word.
                // Undo that adjustment.
                // The function hasn't started yet.
                // This only happens if f was the
                // start function of a new goroutine
                // that hasn't run yet *and* f takes
                // no arguments and has no results
                // (otherwise it will get wrapped in a
                // closure). In this case, we can't
                // reach into its locals because it
                // doesn't have locals yet, but we
                // also know its argument map is
                // empty.
                // No locals, so also no stack objects
                // Figure out whether the return values are valid.
                // Reflect will update this value after it copies
                // in the return values.
                // argMap.n includes the results, but
                // those aren't valid, so drop them.
        return (argMap.clone(), (*hasReflectStackObj.lock().unwrap().as_ref().unwrap()));
    }

    /// getStackMap returns the locals and arguments live pointer maps, and
    /// stack object list for frame.
    pub fn get_stack_map(&self, debug_local: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<crate::stack::bitvector>>>, Arc<Mutex<Option<crate::stack::bitvector>>>, Arc<Mutex<Option<Vec<crate::stack::stackObjectRecord>>>>) {
    let mut locals: Arc<Mutex<Option<bitvector>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut args: Arc<Mutex<Option<bitvector>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut objs: Arc<Mutex<Option<Vec<stackObjectRecord>>>> = Arc::new(Mutex::new(None));

        let mut targetpc = Arc::new(Mutex::new(Some({ let __selector_holder = self.continpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*targetpc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Frame is dead. Return empty bitvectors.
        return (locals.clone(), args.clone(), objs.clone());
    }
                // Frame is dead. Return empty bitvectors.
        let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = self.r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut pcdata = Arc::new(Mutex::new(Some(-(1) as i32)));
        if { let __tmp_x = { let __v = (*targetpc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x != __tmp_y } {
                // Back up to the CALL. If we're at the function entry
                // point, we want to use the entry map (-1), even if
                // the first instruction of the function changes the
                // stack map.
        { let mut guard = targetpc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let new_val = pcdatavalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::P_C_D_A_T_A__STACK_MAP_INDEX as u32))), Arc::new(Mutex::new(Some({ let __arg_holder = targetpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *pcdata.lock().unwrap() = Some(new_val); };
    }
                // Back up to the CALL. If we're at the function entry
                // point, we want to use the entry map (-1), even if
                // the first instruction of the function changes the
                // stack map.
        if { let __tmp_x = { let __v = (*pcdata.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1 as i32; __tmp_x == __tmp_y } {
                // We do not have a valid pcdata value but there might be a
                // stackmap for this function. It is likely that we are looking
                // at the function prologue, assume so and hope for the best.
        { let new_val = 0 as i32; *pcdata.lock().unwrap() = Some(new_val); };
    }
                // We do not have a valid pcdata value but there might be a
                // stackmap for this function. It is likely that we are looking
                // at the function prologue, assume so and hope for the best.
                // Local variables.
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.varp.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.sp.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        let mut minsize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = internal_goarch::ARCH_FAMILY;
    if _switch_val == (internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::A_R_M64 as i32))))) {
            { let new_val = internal_runtime_sys::STACK_ALIGN as usize; *minsize.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = internal_runtime_sys::MIN_FRAME_SIZE as usize; *minsize.lock().unwrap() = Some(new_val); };
        }
    }
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*minsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut stackid = { let __owned = pcdata.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut stkmap: GoPtr<crate::symtab::stackmap> = GoPtr::raw({ let __ptr = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__LOCALS_POINTER_MAPS as u8)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if stkmap.is_nil() || { let __tmp_x = (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: frame ".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " untyped locals ".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.varp.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y } as u64)))));
            let __go_print_arg_4 = format!("{}", "+".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("missing stackmap".to_string()))));
    }
                // If nbit == 0, there's no work to do.
        if { let __tmp_x = (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().nbit.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __v = (*stackid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*stackid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // don't know where we are
        {
            let __go_print_arg_0 = format!("{}", "runtime: pcdata is ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*stackid.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " and ".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " locals stack map entries for ".to_string());
            let __go_print_arg_5 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " (targetpc=".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*targetpc.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_8 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        throw(Arc::new(Mutex::new(Some("bad symbol table".to_string()))));
    }
                // don't know where we are
        { let new_val = stackmapdata(stkmap.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = stackid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *locals.lock().unwrap() = __moved_val; };
        if { let __tmp_x = STACK_DEBUG; let __tmp_y = 3; __tmp_x >= __tmp_y } && { let __v = (*debug_local.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", "      locals ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*stackid.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "/".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = stkmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " ".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*locals.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " words ".to_string());
            let __go_print_arg_7 = format!("{}", { let __ptr = (*locals.lock().unwrap().as_ref().unwrap()).bytedata.clone(); format!("0x{:x}", __ptr.addr()) });
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    }
    } else if { let __tmp_x = STACK_DEBUG; let __tmp_y = 3; __tmp_x >= __tmp_y } && { let __v = (*debug_local.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", "      no locals to adjust\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    }
                // If nbit == 0, there's no work to do.
                // don't know where we are
                // Arguments. First fetch frame size and special-case argument maps.
        let mut isReflect: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = self.arg_map_internal(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *args.lock().unwrap() = __moved_tmp_0; *isReflect.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = (*{ let __field = (*args.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && { let __ptr_field = (*args.lock().unwrap().as_ref().unwrap()).bytedata.clone(); __ptr_field.is_nil() } {
                // Non-empty argument frame, but not a special map.
                // Fetch the argument map at pcdata.
        let mut stackmap: GoPtr<crate::symtab::stackmap> = GoPtr::raw({ let __ptr = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__ARGS_POINTER_MAPS as u8)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if stackmap.is_nil() || { let __tmp_x = (*{ let __ptr_value = stackmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: frame ".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " untyped args ".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = self.argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", "+".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*args.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as i32; __tmp_x * __tmp_y } as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("missing stackmap".to_string()))));
    }
        if { let __tmp_x = { let __v = (*pcdata.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*pcdata.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = stackmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // don't know where we are
        {
            let __go_print_arg_0 = format!("{}", "runtime: pcdata is ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*pcdata.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " and ".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = stackmap.borrow(); __ptr_value.as_ref().unwrap().n.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " args stack map entries for ".to_string());
            let __go_print_arg_5 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " (targetpc=".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*targetpc.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_8 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        throw(Arc::new(Mutex::new(Some("bad symbol table".to_string()))));
    }
                // don't know where we are
        if { let __tmp_x = (*{ let __ptr_value = stackmap.borrow(); __ptr_value.as_ref().unwrap().nbit.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let new_val = 0 as i32; *(*args.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = stackmapdata(stackmap.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pcdata.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *args.lock().unwrap() = __moved_val; };
    }
    }
                // Non-empty argument frame, but not a special map.
                // Fetch the argument map at pcdata.
                // don't know where we are
                // stack objects.
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __go_cond_3 = {
                            let __go_cond_4 = {
                                let __go_cond_5 = {
                                    let __go_cond_6 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "amd64".to_string(); __tmp_x == __tmp_y };
                                    if __go_cond_6 {
                                        true
                                    } else {
                                        let __go_cond_7 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y };
                                        __go_cond_7
                                    }
                                };
                                if __go_cond_5 {
                                    true
                                } else {
                                    let __go_cond_8 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "loong64".to_string(); __tmp_x == __tmp_y };
                                    __go_cond_8
                                }
                            };
                            if __go_cond_4 {
                                true
                            } else {
                                let __go_cond_9 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "ppc64".to_string(); __tmp_x == __tmp_y };
                                __go_cond_9
                            }
                        };
                        if __go_cond_3 {
                            true
                        } else {
                            let __go_cond_10 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "ppc64le".to_string(); __tmp_x == __tmp_y };
                            __go_cond_10
                        }
                    };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_11 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "riscv64".to_string(); __tmp_x == __tmp_y };
                        __go_cond_11
                    }
                };
                if __go_cond_1 {
                    let __go_cond_12 = { let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<internal_abi::r#mod::RegArgs>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = 0 as usize; __tmp_x > __tmp_y };
                    __go_cond_12
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_13 = { let __v = (*isReflect.lock().unwrap().as_ref().unwrap()).clone(); __v };
                __go_cond_13
            } else {
                false
            }
        } {
                // For reflect.makeFuncStub and reflect.methodValueCall,
                // we need to fake the stack object record.
                // These frames contain an internal/abi.RegArgs at a hard-coded offset.
                // This offset matches the assembly code on amd64 and arm64.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = methodValueCallFrameObjs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); objs = new_val; };
    } else {
        let mut p = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__STACK_OBJECTS as u8))));
        if { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
        let mut n = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = p.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = add(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p.lock().unwrap() = __moved_val; };
        let mut r0: GoPtr<crate::stack::stackObjectRecord> = GoPtr::raw({ let __ptr = noescape(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __go_unsafe_result: Arc<Mutex<Option<Vec<crate::stack::stackObjectRecord>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result }; objs = new_val; };
    }
    }
                // For reflect.makeFuncStub and reflect.methodValueCall,
                // we need to fake the stack object record.
                // These frames contain an internal/abi.RegArgs at a hard-coded offset.
                // This offset matches the assembly code on amd64 and arm64.
                // Note: the noescape above is needed to keep
                // getStackMap from "leaking param content:
                // frame".  That leak propagates up to getgcmask, then
                // GCMask, then verifyGCInfo, which converts the stack
                // gcinfo tests into heap gcinfo tests :(
        (locals.clone(), args.clone(), objs.clone())
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for stkframe {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for reflectMethodValue {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
