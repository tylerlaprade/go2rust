use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
use crate::call::*;
use crate::chan::*;
use crate::check::*;
use crate::r#const::*;
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
use crate::format::*;
use crate::gccgosizes::*;
use crate::gcsizes::*;
use crate::index::*;
use crate::infer::*;
use crate::initorder::*;
use crate::instantiate::*;
use crate::interface::*;
use crate::iter::*;
use crate::labels::*;
use crate::literals::*;
use crate::lookup::*;
use crate::map::*;
use crate::methodset::*;
use crate::mono::*;
use crate::named::*;
use crate::object::*;
use crate::objset::*;
use crate::operand::*;
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
use crate::resolver::*;
use crate::r#return::*;
use crate::scope::*;
use crate::scope2::*;
use crate::selection::*;
use crate::signature::*;
use crate::sizes::*;
use crate::slice::*;
use crate::stmt::*;
use crate::r#struct::*;
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// TypeParamList holds a list of type parameters.
#[derive(Clone, Default)]
pub struct TypeParamList {
    pub tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>,
}

impl TypeParamList {
    pub fn __go_value_clone(&self) -> Self {
        Self { tparams: self.tparams.clone() }
    }
}

impl std::fmt::Display for TypeParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped(&self.tparams))
    }
}

impl GoJsonDecode for TypeParamList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// TypeList holds a list of types.
#[derive(Clone, Default)]
pub struct TypeList {
    pub types: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>,
}

impl TypeList {
    pub fn __go_value_clone(&self) -> Self {
        Self { types: self.types.clone() }
    }
}

impl std::fmt::Display for TypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped_stringer(&self.types))
    }
}

impl GoJsonDecode for TypeList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<go_ast::r#mod::Ident>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), ptr: { let __guard = self.ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: self.recv.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { obj: Arc::new(Mutex::new(None)), ptr: Arc::new(Mutex::new(Some(false))), recv: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.obj.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ptr.lock().unwrap().as_ref().unwrap()), { let __guard = self.recv.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl TypeParamList {
    /// Len returns the number of type parameters in the list.
    /// It is safe to call on a nil receiver.
    pub fn len(&self) -> i32 {
        (*self.list().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
    }

    /// At returns the i'th type parameter in the list.
    pub fn at(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::typeparam::TypeParam>>> {
        { let __seq = { let __seq_holder = self.tparams.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// list is for internal use where we expect a []*TypeParam.
    /// TODO(rfindley): list should probably be eliminated: we can pass around a
    /// TypeParamList instead.
    pub fn list(&self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>>>> {
        if false {
        return Arc::new(Mutex::new(None));
    }
        return self.tparams.clone();
    }
}

impl TypeList {
    /// Len returns the number of types in the list.
    /// It is safe to call on a nil receiver.
    pub fn len(&self) -> i32 {
        (*self.list().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
    }

    /// At returns the i'th type in the list.
    pub fn at(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        { let __seq = { let __seq_holder = self.types.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// list is for internal use where we expect a []Type.
    /// TODO(rfindley): list should probably be eliminated: we can pass around a
    /// TypeList instead.
    pub fn list(&self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> {
        if false {
        return Arc::new(Mutex::new(None));
    }
        return self.types.clone();
    }
}

/// newTypeList returns a new TypeList with the types in list.
pub fn new_type_list(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<TypeList>>> {
    if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some(TypeList { types: list.clone(), ..Default::default() })))
}

pub fn bind_t_params(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) -> Arc<Mutex<Option<TypeParamList>>> {
    if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, typ) in __range_values.iter().enumerate() {
        if { let __tmp_x = (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        std::panic::panic_any(Box::new("type parameter bound more than once".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let new_val = i as i32; *(*typ.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
    } }
    Arc::new(Mutex::new(Some(TypeParamList { tparams: list.clone(), ..Default::default() })))
}

impl GoValueClone for TypeParamList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
