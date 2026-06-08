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

use crate::{symtab::{funcInfo, funcdata, funcline1, moduledata, pcdatavalue1, srcFunc}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// inlinedCall is the encoding of entries in the FUNCDATA_InlTree table.
#[derive(Clone)]
pub struct inlinedCall {
    pub func_i_d: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>,
    pub __blank_1_0: Arc<Mutex<Option<[u8; 3]>>>,
    pub name_off: Arc<Mutex<Option<i32>>>,
    pub parent_pc: Arc<Mutex<Option<i32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
}

impl inlinedCall {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.func_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.name_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.parent_pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            func_i_d: __go_clone_0_0,
            __blank_1_0: __go_clone_1_0,
            name_off: __go_clone_2_0,
            parent_pc: __go_clone_3_0,
            start_line: __go_clone_4_0,
        }
    }
}


impl Default for inlinedCall {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            func_i_d: __go_default_0_0,
            __blank_1_0: __go_default_1_0,
            name_off: __go_default_2_0,
            parent_pc: __go_default_3_0,
            start_line: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for inlinedCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.func_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.__blank_1_0));
        let __go_fmt_2 = format!("{}", (*self.name_off.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.parent_pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.start_line.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for inlinedCall {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An inlineUnwinder iterates over the stack of inlined calls at a PC by
/// decoding the inline table. The last step of iteration is always the frame of
/// the physical function, so there's always at least one frame.
///
/// This is typically used as:
///
///	for u, uf := newInlineUnwinder(...); uf.valid(); uf = u.next(uf) { ... }
///
/// Implementation note: This is used in contexts that disallow write barriers.
/// Hence, the constructor returns this by value and pointer receiver methods
/// must not mutate pointer fields. Also, we keep the mutable state in a separate
/// struct mostly to keep both structs SSA-able, which generates much better
/// code.
#[derive(Clone)]
pub struct inlineUnwinder {
    pub f: Arc<Mutex<Option<funcInfo>>>,
    pub inl_tree: GoPtr<[inlinedCall; 1048576]>,
}

impl inlineUnwinder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.f.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.inl_tree.clone();
        Self {
            f: __go_clone_0_0,
            inl_tree: __go_clone_1_0,
        }
    }
}


impl Default for inlineUnwinder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(funcInfo::default())));
        let __go_default_1_0 = GoPtr::nil();
        Self {
            f: __go_default_0_0,
            inl_tree: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for inlineUnwinder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.f.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { if self.inl_tree.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for inlineUnwinder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An inlineFrame is a position in an inlineUnwinder.
#[derive(Debug, Clone)]
pub struct inlineFrame {
    pub pc: Arc<Mutex<Option<usize>>>,
    pub index: Arc<Mutex<Option<i32>>>,
}

impl inlineFrame {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            pc: __go_clone_0_0,
            index: __go_clone_1_0,
        }
    }
}


impl Default for inlineFrame {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            pc: __go_default_0_0,
            index: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for inlineFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.index.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for inlineFrame {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl inlineUnwinder {
    pub fn resolve_internal(&self, pc: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<inlineFrame>>> {
        Arc::new(Mutex::new(Some(inlineFrame { pc: Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: Arc::new(Mutex::new(Some(pcdatavalue1(Arc::new(Mutex::new(Some({ let __selector_holder = self.f.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(internal_abi::P_C_D_A_T_A__INL_TREE_INDEX as u32))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))))))), ..Default::default() })))
    }

    /// next returns the frame representing uf's logical caller.
    pub fn next(&self, uf: Arc<Mutex<Option<inlineFrame>>>) -> Arc<Mutex<Option<inlineFrame>>> {
        if { let __tmp_x = (*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let new_val = 0 as usize; *(*uf.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        return { let __owned = uf.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        let mut parentPc = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = self.inl_tree.clone().borrow(); __seq.as_ref().unwrap()[((*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize].clone() }.parent_pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        return { let __method_arg0 = Arc::new(Mutex::new(Some({
            let __tmp_x = (*self.f.lock().unwrap().as_ref().unwrap()).entry();
            let __tmp_y = (*Arc::new(Mutex::new(Some((*parentPc.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        }))); self.resolve_internal(__method_arg0) };
    }

    /// isInlined returns whether uf is an inlined frame.
    pub fn is_inlined(&self, uf: Arc<Mutex<Option<inlineFrame>>>) -> bool {
        return { let __tmp_x = (*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x >= __tmp_y };
    }

    /// srcFunc returns the srcFunc representing the given frame.
    ///
    /// srcFunc should be an internal detail,
    /// but widely used packages access it using linkname.
    /// Notable members of the hall of shame include:
    ///   - github.com/phuslu/log
    ///
    /// Do not remove or change the type signature.
    /// See go.dev/issue/67401.
    ///
    /// The go:linkname is below.
    pub fn src_func(&self, uf: Arc<Mutex<Option<inlineFrame>>>) -> Arc<Mutex<Option<crate::symtab::srcFunc>>> {
        if { let __tmp_x = (*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        return (*self.f.lock().unwrap().as_ref().unwrap()).src_func();
    }
        let mut t: Option<GoArrayElemPtr<inlinedCall, 1048576>> = Some(GoArrayElemPtr::from_go_ptr(self.inl_tree.clone(), ((*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize));
        return Arc::new(Mutex::new(Some(srcFunc {
            datap: { let __field = (*self.f.lock().unwrap().as_ref().unwrap()).datap.clone(); __field },
            name_off: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.as_ref().unwrap().borrow().as_ref().unwrap()).name_off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            start_line: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.as_ref().unwrap().borrow().as_ref().unwrap()).start_line.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            func_i_d: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.as_ref().unwrap().borrow().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            ..Default::default()
        })));
    }

    /// fileLine returns the file name and line number of the call within the given
    /// frame. As a convenience, for the innermost frame, it returns the file and
    /// line of the PC this unwinder was started at (often this is a call to another
    /// physical function).
    ///
    /// It returns "?", 0 if something goes wrong.
    pub fn file_line(&self, uf: Arc<Mutex<Option<inlineFrame>>>) -> (Arc<Mutex<Option<String>>>, i32) {
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        let (__tmp_0, mut line32) = funcline1(Arc::new(Mutex::new(Some({ let __selector_holder = self.f.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*uf.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *file.lock().unwrap() = __moved_tmp_0;;
        return ({ let __owned = file.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, (*Arc::new(Mutex::new(Some(line32 as i32))).lock().unwrap().as_ref().unwrap()));
    }
}

impl inlineFrame {
    pub fn valid(&self) -> bool {
        return { let __tmp_x = (*self.pc.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
    }
}

/// newInlineUnwinder creates an inlineUnwinder initially set to the inner-most
/// inlined frame at PC. PC should be a "call PC" (not a "return PC").
///
/// This unwinder uses non-strict handling of PC because it's assumed this is
/// only ever used for symbolic debugging. If things go really wrong, it'll just
/// fall back to the outermost frame.
///
/// newInlineUnwinder should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/phuslu/log
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname newInlineUnwinder
pub fn new_inline_unwinder(f: Arc<Mutex<Option<funcInfo>>>, pc: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<inlineUnwinder>>>, Arc<Mutex<Option<inlineFrame>>>) {
    let mut inldata = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__INL_TREE as u8))));
    if { let __nil_result = (*inldata.lock().unwrap()).is_none(); __nil_result } {
        return (Arc::new(Mutex::new(Some(inlineUnwinder { f: Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), Arc::new(Mutex::new(Some(inlineFrame { pc: Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: Arc::new(Mutex::new(Some(-1 as i32))), ..Default::default() }))));
    }
    let mut inlTree: GoPtr<[inlinedCall; 1048576]> = GoPtr::raw({ let __ptr = inldata.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut u = Arc::new(Mutex::new(Some(inlineUnwinder { f: Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), inl_tree: inlTree.clone(), ..Default::default() })));
    return ({ let __owned = u.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, (*u.lock().unwrap().as_ref().unwrap()).resolve_internal(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
}

impl GoValueClone for inlinedCall {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for inlineUnwinder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for inlineFrame {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
