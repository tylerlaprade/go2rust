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
    iface::{unreachable_method},
    lock_spinbit::{lock, unlock},
    malloc::{persistentalloc},
    mbitmap::{addb},
    mstats::{memstats, sysMemStat},
    os_darwin::{osyield},
    panic::{throw},
    print::{hex},
    r#extern::{G_O_O_S},
    race0::{RACEENABLED, raceacquire, racerelease},
    runtime2::{mutex},
    stubs::{add, div_round_up, systemstack},
    symtab::{aixStaticDataBase, firstmoduledata, moduledata},
};

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub type nameOff = Arc<Mutex<Option<internal_abi::r#type::NameOff>>>;


pub type typeOff = Arc<Mutex<Option<internal_abi::r#type::TypeOff>>>;


pub type textOff = Arc<Mutex<Option<internal_abi::r#type::TextOff>>>;


pub type _type = Arc<Mutex<Option<internal_abi::r#type::Type>>>;


/// rtype is a wrapper that allows us to define additional methods.
#[derive(Clone, Default)]
pub struct rtype {
    pub r#type: GoPtr<internal_abi::r#type::Type>,
}

impl rtype {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r#type.clone();
        Self {
            r#type: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for rtype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.r#type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// A bitCursor is a simple cursor to memory to which we
/// can write a set of bits.
#[derive(Debug, Clone)]
pub struct bitCursor {
    pub ptr: GoPtr<u8>,
    pub n: Arc<Mutex<Option<usize>>>,
}

impl bitCursor {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.ptr.clone();
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            ptr: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}


impl Default for bitCursor {
    fn default() -> Self {
        let __go_default_0_0 = GoPtr::nil();
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            ptr: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for bitCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.ptr.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub type uncommontype = Arc<Mutex<Option<internal_abi::r#type::UncommonType>>>;


pub type interfacetype = Arc<Mutex<Option<internal_abi::r#type::InterfaceType>>>;


pub type functype = Arc<Mutex<Option<internal_abi::r#type::FuncType>>>;


pub type ptrtype = Arc<Mutex<Option<internal_abi::r#type::PtrType>>>;


pub type name = Arc<Mutex<Option<internal_abi::r#type::Name>>>;


pub type structtype = Arc<Mutex<Option<internal_abi::r#type::StructType>>>;


pub(crate) static inProgress: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static reflectOffs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct39>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *inProgress.lock().unwrap() = Some(0);
    *reflectOffs.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *inProgress.lock().unwrap() = Some(0);
    *reflectOffs.lock().unwrap() = Some(Default::default());
}


impl rtype {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = {
            let __recv = self.name_off(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.str.clone(); __field }); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name();
            __result
        };
        if {
            let __tmp_x = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.t_flag.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_EXTRA_STAR as u8)))); __tmp_x & __tmp_y };
            let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x != __tmp_y
        } {
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })));
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<internal_abi::r#type::UncommonType>>> {
        { let __promoted_recv = self.r#type.clone(); let __result = __promoted_recv.with_mut(|__promoted_ref| { __promoted_ref.uncommon() }); __result }
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if {
            let __tmp_x = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.t_flag.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_NAMED as u8)))); __tmp_x & __tmp_y };
            let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut s = self.string();
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut sqBrackets = Arc::new(Mutex::new(Some(0)));
        while {
            let __go_cond_0 = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] };
                        let __tmp_y = ('.' as i32) as u8;
                        __tmp_x != __tmp_y
                    };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_3 = { let __tmp_x = { let __v = (*sqBrackets.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y };
                        __go_cond_3
                    }
                };
                __go_cond_1
            } else {
                false
            }
        } {
        { let _switch_val = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] };
    if _switch_val == ((']' as i32) as u8) {
            { let mut guard = sqBrackets.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if _switch_val == (('[' as i32) as u8) {
            { let mut guard = sqBrackets.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })));
    }

    /// pkgpath returns the path of the package where t was defined, if
    /// available. This is not the same as the reflect package's PkgPath
    /// method, in that it returns the package path for struct and interface
    /// types, not just named types.
    pub fn pkgpath(&self) -> Arc<Mutex<Option<String>>> {
        {
        let mut u = self.uncommon();;
        if { let __nil_result = (*u.lock().unwrap()).is_some(); __nil_result } {
            return {
                let __recv = self.name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).pkg_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
                let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name();
                __result
            };;
        }
    }
        { let _switch_val = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.kind_.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y };
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::STRUCT as u8))))) {
            let mut st: GoPtr<internal_abi::r#type::StructType> = { let __ptr = Arc::new(Mutex::new(Some(self.r#type.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::StructType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::StructType")) } };
            return (*{ let __ptr_value = st.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name();
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::INTERFACE as u8))))) {
            let mut it: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(self.r#type.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
            return (*{ let __ptr_value = it.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name();
        }
    }
        Arc::new(Mutex::new(Some("".to_string())))
    }

    pub fn name_off(&self, off: nameOff) -> Arc<Mutex<Option<internal_abi::r#type::Name>>> {
        resolve_name_off(
            Arc::new(Mutex::new(Some(self.r#type.addr()))),
            Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        )
    }

    pub fn type_off(&self, off: typeOff) -> GoPtr<internal_abi::r#type::Type> {
        resolve_type_off(
            Arc::new(Mutex::new(Some(self.r#type.addr()))),
            Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        )
    }

    pub fn text_off(&self, off: textOff) -> Arc<Mutex<Option<usize>>> {
        if {
            let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_y = internal_abi::r#type::TextOff(Arc::new(Mutex::new(Some(-1 as i32))));
            __tmp_x == __tmp_y
        } {
                // -1 is the sentinel value for unreachable code.
                // See cmd/link/internal/ld/data.go:relocsym.
        return Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(unreachable_method.clone()) as Box<dyn Any + Send + Sync>)))))));
    }
                // -1 is the sentinel value for unreachable code.
                // See cmd/link/internal/ld/data.go:relocsym.
        let mut base = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self.r#type.addr()))).lock().unwrap().as_ref().unwrap()) as usize)));
        let mut md: Arc<Mutex<Option<moduledata>>> = Arc::new(Mutex::new(None));
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = next.clone(); md = new_val; };
        break
    }
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        if { let __nil_result = (*md.lock().unwrap()).is_none(); __nil_result } {
        reflect_offs_lock();
        let mut res = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*reflectOffs.lock().unwrap().as_ref().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __v = Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        reflect_offs_unlock();
        if { let __nil_result = (*res.lock().unwrap()).is_none(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: textOff".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "base".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", "not in ranges:".to_string());
            eprintln!("{} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "\ttypes".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", "etypes".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        throw(Arc::new(Mutex::new(Some("runtime: text offset base pointer out of range".to_string()))));
    }
        return { let __owned = res.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        let mut res = { let __recv = md.clone(); let __recv_ptr: *const crate::symtab::moduledata = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::symtab::moduledata }; let __result = unsafe { &*__recv_ptr }.text_addr(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32)))); __result };
        Arc::new(Mutex::new(Some(res)))
    }

    pub fn align(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.align() })
    }

    pub fn array_type(&self) -> GoPtr<internal_abi::r#type::ArrayType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { {
            let __go_ptr = embedded_ref.array_type().clone();
            match __go_ptr {
                internal_abi::GoPtr::Nil => GoPtr::nil(),
                internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        } })
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<internal_abi::r#type::ChanDir>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.chan_dir() })
    }

    pub fn common(&self) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.common() })
    }

    pub fn elem(&self) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.elem() })
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<internal_abi::r#type::Method>>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.exported_methods() })
    }

    pub fn field_align(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.field_align() })
    }

    pub fn func_type(&self) -> GoPtr<internal_abi::r#type::FuncType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { {
            let __go_ptr = embedded_ref.func_type().clone();
            match __go_ptr {
                internal_abi::GoPtr::Nil => GoPtr::nil(),
                internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        } })
    }

    pub fn gc_slice(&self, _arg0: Arc<Mutex<Option<usize>>>, _arg1: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.gc_slice(_arg0, _arg1) })
    }

    pub fn has_name(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.has_name() })
    }

    pub fn iface_indir(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.iface_indir() })
    }

    pub fn interface_type(&self) -> GoPtr<internal_abi::r#type::InterfaceType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { {
            let __go_ptr = embedded_ref.interface_type().clone();
            match __go_ptr {
                internal_abi::GoPtr::Nil => GoPtr::nil(),
                internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        } })
    }

    pub fn is_direct_iface(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_direct_iface() })
    }

    pub fn key(&self) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.key() })
    }

    pub fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.kind() })
    }

    pub fn len(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.len() })
    }

    pub fn map_type(&self) -> GoPtr<internal_abi::map_swiss::SwissMapType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { {
            let __go_ptr = embedded_ref.map_type().clone();
            match __go_ptr {
                internal_abi::GoPtr::Nil => GoPtr::nil(),
                internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        } })
    }

    pub fn num_method(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.num_method() })
    }

    pub fn pointers(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.pointers() })
    }

    pub fn size(&self) -> usize {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.size() })
    }

    pub fn struct_type(&self) -> GoPtr<internal_abi::r#type::StructType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { {
            let __go_ptr = embedded_ref.struct_type().clone();
            match __go_ptr {
                internal_abi::GoPtr::Nil => GoPtr::nil(),
                internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        } })
    }
}

impl bitCursor {
    /// Write to b cnt bits starting at bit 0 of data.
    /// Requires cnt>0.
    pub fn write(&self, mut data: GoPtr<u8>, mut cnt: Arc<Mutex<Option<usize>>>) {
                // Starting byte for writing.
        let mut p: GoPtr<u8> = addb(
            self.ptr.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))
        );
                // Note: if we're starting halfway through a byte, we load the
                // existing lower bits so we don't clobber them.
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as usize; __tmp_x % __tmp_y })));
        let mut buf_local = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __ptr_value = p.borrow(); __ptr_value.as_ref().unwrap().clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })));
                // Work 8 bits at a time.
        while { let __tmp_x = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x > __tmp_y } {
                // Read 8 more bits, now buf has 8-15 valid bits in it.
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __ptr_value = data.borrow(); __ptr_value.as_ref().unwrap().clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = 8 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        data = addb(data.clone(), Arc::new(Mutex::new(Some(1 as usize))));
        { let __rhs = 8 as usize; let mut guard = cnt.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

                // Write 8 of the buffered bits out.
        { let new_val = (*Arc::new(Mutex::new(Some((*buf_local.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.assign(Some(new_val)); };
        { let __rhs = 8 as usize; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 8 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        p = addb(p.clone(), Arc::new(Mutex::new(Some(1 as usize))));
    }
                // Read 8 more bits, now buf has 8-15 valid bits in it.
                // Write 8 of the buffered bits out.
                // Read remaining bits.
        { let __rhs = { let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __ptr_value = data.borrow(); __ptr_value.as_ref().unwrap().clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = (*cnt.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Flush remaining bits.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x > __tmp_y } {
        { let new_val = (*Arc::new(Mutex::new(Some((*buf_local.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.assign(Some(new_val)); };
        { let __rhs = 8 as usize; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 8 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        p = addb(p.clone(), Arc::new(Mutex::new(Some(1 as usize))));
    }
        { let __rhs = { let __tmp_x = { let __tmp_x = (1 as u8); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as u8; __tmp_x - __tmp_y }; p.with_mut(|__ptr_value| { *__ptr_value = __ptr_value.clone() & ! __rhs; }); };
        { let __rhs = (*Arc::new(Mutex::new(Some((*buf_local.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); p.with_mut(|__ptr_value| { *__ptr_value = __ptr_value.clone() | __rhs; }); };
    }

    pub fn offset(&self, cnt: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<bitCursor>>> {
        Arc::new(Mutex::new(Some(bitCursor { ptr: self.ptr.clone(), n: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), ..Default::default() })))
    }
}

/// getGCMask returns the pointer/nonpointer bitmask for type t.
///
/// nosplit because it is used during write barriers and must not be preempted.
///
///go:nosplit
pub fn get_g_c_mask(t: GoPtr<internal_abi::r#type::Type>) -> GoPtr<u8> {
    if {
        let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.t_flag.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_G_C_MASK_ON_DEMAND as u8)))); __tmp_x & __tmp_y };
        let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8))));
        __tmp_x != __tmp_y
    } {
                // Split the rest into getGCMaskOnDemand so getGCMask itself is inlineable.
        return get_g_c_mask_on_demand(t.clone());
    }
        // Split the rest into getGCMaskOnDemand so getGCMask itself is inlineable.
    {
        let __go_ptr = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.g_c_data.clone()); __ptr_value }.clone();
        match __go_ptr {
            internal_abi::GoPtr::Nil => GoPtr::nil(),
            internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
            internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
            internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
            internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
        }
    }
}

/// nosplit because it is used during write barriers and must not be preempted.
///
///go:nosplit
pub fn get_g_c_mask_on_demand(t: GoPtr<internal_abi::r#type::Type>) -> GoPtr<u8> {
        // For large types, GCData doesn't point directly to a bitmask.
        // Instead it points to a pointer to a bitmask, and the runtime
        // is responsible for (on first use) creating the bitmask and
        // storing a pointer to it in that slot.
        // TODO: we could use &t.GCData as the slot, but types are
        // in read-only memory currently.
    let mut addr = Arc::new(Mutex::new(Some({ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.g_c_data.clone()); __ptr_value }.addr())));

    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "aix".to_string(); __tmp_x == __tmp_y } {
        { let new_val = add(
            Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*firstmoduledata.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*aixStaticDataBase.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })))
        ); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *addr.lock().unwrap() = __moved_val; };
    }

    loop {
        let mut p: GoPtr<u8> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if { let __switch_val = p.clone(); { let __case = GoPtr::local(inProgress.clone()); GoPtr::ptr_eq(&__switch_val, &__case) } } {
                        // Just wait until the builder is done.
                        // We can't block here, so spinning while having
                        // the OS thread yield is about the best we can do.
            osyield();
            continue
        } else if { let __switch_val = p.clone(); __switch_val.is_nil() } {
                        // Attempt to get exclusive access to build it.
            if !internal_runtime_atomic::casp1(internal_runtime_atomic::GoPtr::raw({ let __ptr = addr.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Arc::as_ptr(&inProgress.clone()) as usize)))) {
        continue
    }
                        // Build gcmask for this type.
            let mut bytes = Arc::new(Mutex::new(Some({
                let __tmp_x = internal_goarch::PTR_SIZE as usize;
                let __tmp_y =
                    div_round_up(
                        Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }))),
                        Arc::new(Mutex::new(Some(((8 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize)))
                    );
                __tmp_x * __tmp_y
            })));
            p = GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            let p_closure_clone = p.clone(); let t_closure_clone = t.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        build_g_c_mask(
            t_closure_clone.clone(),
            Arc::new(Mutex::new(Some(bitCursor { ptr: p_closure_clone.clone(), n: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() })))
        );
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                        // Store the newly-built gcmask for future callers.
            internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(p.addr()))));
            return p.clone();
        } else {
            return p.clone();
        }
    }
}

/// buildGCMask writes the ptr/nonptr bitmap for t to dst.
/// t must have a pointer.
pub fn build_g_c_mask(mut t: GoPtr<internal_abi::r#type::Type>, mut dst: Arc<Mutex<Option<bitCursor>>>) {
    'top: loop {
                // Note: we want to avoid a situation where buildGCMask gets into a
                // very deep recursion, because M stacks are fixed size and pretty small
                // (16KB). We do that by ensuring that any recursive
                // call operates on a type at most half the size of its parent.
                // Thus, the recursive chain can be at most 64 calls deep (on a
                // 64-bit machine).
                // Recursion is avoided by using a "tail call" (jumping to the
                // "top" label) for any recursive call with a large subtype.
        if { let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("pointerless type".to_string()))));
    }
        if {
            let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.t_flag.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_G_C_MASK_ON_DEMAND as u8)))); __tmp_x & __tmp_y };
            let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        } {
                // copy t.GCData to dst
        (*dst.lock().unwrap().as_ref().unwrap()).write(
            {
                let __go_ptr = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.g_c_data.clone()); __ptr_value }.clone();
                match __go_ptr {
                    internal_abi::GoPtr::Nil => GoPtr::nil(),
                    internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                    internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                    internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
                }
            },
            Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }))),
        );
        return;
    }

                // copy t.GCData to dst
                // The above case should handle all kinds except
                // possibly arrays and structs.
        { let _switch_val = { let __v = { let __recv_value = t.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::ARRAY as u8))))) {
            let mut a: GoPtr<internal_abi::r#type::ArrayType> = {
                let __go_ptr = { let __result = t.with_mut(|__recv_value| __recv_value.array_type()); __result }.clone();
                match __go_ptr {
                    internal_abi::GoPtr::Nil => GoPtr::nil(),
                    internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                    internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                    internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
                }
            };
            if { let __tmp_x = (*{ let __ptr_value = a.borrow(); __ptr_value.as_ref().unwrap().len.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
                // Avoid recursive call for element type that
                // isn't smaller than the parent type.
        t = GoPtr::local({ let __ptr_value = a.borrow(); let __field_value = __ptr_value.as_ref().unwrap().elem.clone(); __field_value });
        continue 'top;
    }
                        // Avoid recursive call for element type that
                        // isn't smaller than the parent type.
            let mut e = { let __ptr_value = a.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
            let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = a.borrow(); __ptr_value.as_ref().unwrap().len.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        build_g_c_mask(GoPtr::local(e.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*dst.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dst.lock().unwrap() = __moved_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::STRUCT as u8))))) {
            let mut s: GoPtr<internal_abi::r#type::StructType> = {
                let __go_ptr = { let __result = t.with_mut(|__recv_value| __recv_value.struct_type()); __result }.clone();
                match __go_ptr {
                    internal_abi::GoPtr::Nil => GoPtr::nil(),
                    internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                    internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                    internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    internal_abi::GoPtr::ArrayElem(__value) => GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
                }
            };
            let mut bigField: Arc<Mutex<Option<internal_abi::r#type::StructField>>> = Arc::new(Mutex::new(Some(Default::default())));
            { let __range_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        let mut ft = f.typ.clone();
        if !{ let __recv = ft.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.pointers(); __result } {
        continue
    }
        if {
            let __tmp_x = (*{ let __field = (*ft.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap());
            let __tmp_y = { let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x / __tmp_y };
            __tmp_x > __tmp_y
        } {
                // Avoid recursive call for field type that
                // is larger than half of the parent type.
                // There can be only one.
        { let new_val = (*f).clone(); *bigField.lock().unwrap() = Some(new_val); };
        continue
    }
                // Avoid recursive call for field type that
                // is larger than half of the parent type.
                // There can be only one.
        build_g_c_mask(
            GoPtr::local(ft.clone()),
            (*dst.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __tmp_x = (*f.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }))))
        );
    } }
                        // Avoid recursive call for field type that
                        // is larger than half of the parent type.
                        // There can be only one.
            if { let __nil_target = (*bigField.lock().unwrap().as_ref().unwrap()).typ.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Note: this case causes bits to be written out of order.
        t = GoPtr::local((*bigField.lock().unwrap().as_ref().unwrap()).typ.clone());
        { let new_val = (*dst.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*bigField.lock().unwrap().as_ref().unwrap()).offset.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dst.lock().unwrap() = __moved_val; };
        continue 'top;
    }
        } else {
            throw(Arc::new(Mutex::new(Some("unexpected kind".to_string()))));
        }
    }
        break 'top;
    };
}

pub fn reflect_offs_lock() {
    lock(GoPtr::local((*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()) as usize))));
    }
}

pub fn reflect_offs_unlock() {
    if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()) as usize))));
    }
    unlock(GoPtr::local((*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

pub fn resolve_name_off(ptrInModule: Arc<Mutex<Option<usize>>>, off: nameOff) -> Arc<Mutex<Option<internal_abi::r#type::Name>>> {
    if { let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::NameOff(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(internal_abi::r#type::Name { ..Default::default() })));
    }
    let mut base = Arc::new(Mutex::new(Some((*ptrInModule.lock().unwrap().as_ref().unwrap()) as usize)));
    let mut md = firstmoduledata.clone();
    while { let __nil_result = (*md.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut res = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: nameOff".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "out of range".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", "-".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprintln!("{} {} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5)
        };
        throw(Arc::new(Mutex::new(Some("runtime: name offset out of range".to_string()))));
    }
        return Arc::new(Mutex::new(Some(internal_abi::r#type::Name {
            bytes: internal_abi::GoPtr::local(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*res.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()),
            ..Default::default()
        })));
    }
        { let new_val = (*md.lock().unwrap().as_ref().unwrap()).next.clone(); md = new_val; };
    }

        // No module found. see if it is a run time name.
    reflect_offs_lock();
    let (mut res, mut found) = { let __map = { let __map_holder = (*reflectOffs.lock().unwrap().as_ref().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    reflect_offs_unlock();
    if !found {
        {
            let __go_print_arg_0 = format!("{}", "runtime: nameOff".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "base".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", "not in ranges:".to_string());
            eprintln!("{} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "\ttypes".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", "etypes".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        throw(Arc::new(Mutex::new(Some("runtime: name offset base pointer out of range".to_string()))));
    }
    return Arc::new(Mutex::new(Some(internal_abi::r#type::Name {
        bytes: internal_abi::GoPtr::local(Arc::new(Mutex::new({ let __ptr = res.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()),
        ..Default::default()
    })));
}

pub fn resolve_type_off(ptrInModule: Arc<Mutex<Option<usize>>>, off: typeOff) -> GoPtr<internal_abi::r#type::Type> {
    if {
        let __go_cond_0 = { let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TypeOff(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = {
                let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone();
                let __tmp_y = internal_abi::r#type::TypeOff(Arc::new(Mutex::new(Some(-1 as i32))));
                __tmp_x == __tmp_y
            };
            __go_cond_1
        }
    } {
                // -1 is the sentinel value for unreachable code.
                // See cmd/link/internal/ld/data.go:relocsym.
        return GoPtr::nil();
    }
        // -1 is the sentinel value for unreachable code.
        // See cmd/link/internal/ld/data.go:relocsym.
    let mut base = Arc::new(Mutex::new(Some((*ptrInModule.lock().unwrap().as_ref().unwrap()) as usize)));
    let mut md: Arc<Mutex<Option<moduledata>>> = Arc::new(Mutex::new(None));
    let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = next.clone(); md = new_val; };
        break
    }
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
    if { let __nil_result = (*md.lock().unwrap()).is_none(); __nil_result } {
        reflect_offs_lock();
        let mut res = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*reflectOffs.lock().unwrap().as_ref().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __v = Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        reflect_offs_unlock();
        if { let __nil_result = (*res.lock().unwrap()).is_none(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: typeOff".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "base".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", "not in ranges:".to_string());
            eprintln!("{} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "\ttypes".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", "etypes".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        throw(Arc::new(Mutex::new(Some("runtime: type offset base pointer out of range".to_string()))));
    }
        return GoPtr::raw({ let __ptr = res.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
    {
        let mut t = { let __map = { let __map_holder = (*md.lock().unwrap().as_ref().unwrap()).typemap.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*off.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            return GoPtr::local(t.clone());;
        }
    }
    let mut res = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: typeOff".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "out of range".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", "-".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprintln!("{} {} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5)
        };
        throw(Arc::new(Mutex::new(Some("runtime: type offset out of range".to_string()))));
    }
    return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*res.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
}

pub fn pkg_path(n: name) -> Arc<Mutex<Option<String>>> {
    if { let __ptr_field = (*n.lock().unwrap().as_ref().unwrap()).bytes.clone(); __ptr_field.is_nil() } || { let __tmp_x = { let __tmp_x = { let __ptr_handle = (*n.lock().unwrap().as_ref().unwrap()).data(Arc::new(Mutex::new(Some(0)))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ((1 as u8) << (2 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let (mut i, mut l) = (*n.lock().unwrap().as_ref().unwrap()).read_varint(Arc::new(Mutex::new(Some(1))));
    let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = i; __tmp_x + __tmp_y }; let __tmp_y = l; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __tmp_x = { let __ptr_handle = (*n.lock().unwrap().as_ref().unwrap()).data(Arc::new(Mutex::new(Some(0)))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ((1 as u8) << (1 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        let (mut i2, mut l2) = (*n.lock().unwrap().as_ref().unwrap()).read_varint(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = { let __tmp_x = i2; let __tmp_y = l2; __tmp_x + __tmp_y }; let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut nameOff: nameOff = Arc::new(Mutex::new(Some(internal_abi::r#type::NameOff(Arc::new(Mutex::new(Some(0)))))));
    {
        let _dst_start = 0;
        let _dst_len = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&nameOff.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).lock().unwrap().as_ref().unwrap()).len() - _dst_start;
        let _src = (*Arc::new(Mutex::new(Some({
            let __seq_holder = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).data(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = __seq.len();
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))).lock().unwrap().as_ref().unwrap()).clone();
        let _n = std::cmp::min(_dst_len, _src.len());
        for _i in 0.._n {
            (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&nameOff.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
        }
        Arc::new(Mutex::new(Some(_n as i32)))
    };
    let mut pkgPathName = resolve_name_off(
        Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).bytes.addr()))),
        Arc::new(Mutex::new(Some({ let __arg_holder = nameOff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
    );
    return (*pkgPathName.lock().unwrap().as_ref().unwrap()).name();
}

pub fn to_r_type(t: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<rtype>>> {
    Arc::new(Mutex::new(Some(rtype { r#type: t.clone(), ..Default::default() })))
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct39 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub next: Arc<Mutex<Option<i32>>>,
    pub m: Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<usize>>>>>>>,
    pub minv: Arc<Mutex<Option<BTreeMap<usize, Arc<Mutex<Option<i32>>>>>>>,
}
impl AnonymousStruct39 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.m.clone();
        let __go_clone_3_0 = self.minv.clone();
        Self {
            lock: __go_clone_0_0,
            next: __go_clone_1_0,
            m: __go_clone_2_0,
            minv: __go_clone_3_0,
        }
    }
}


impl Default for AnonymousStruct39 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        Self {
            lock: __go_default_0_0,
            next: __go_default_1_0,
            m: __go_default_2_0,
            minv: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct39 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.next.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_map(&self.m));
        let __go_fmt_3 = format!("{}", format_map(&self.minv));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


pub(crate) type reflectOffs = AnonymousStruct39;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for rtype {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for bitCursor {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
